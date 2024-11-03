use crate::models::access_token::{self, NewAccessToken};
use crate::models::refresh_token::NewRefreshToken;
use crate::models::user::User;
use crate::schema::client::dsl::{client, id};
use crate::schema::user::dsl::{email, user};
use crate::utils::connect_sql::establish_connection;
use crate::utils::generate_random_hash::generate_random_hash_function;
use crate::utils::generate_short_hash::encrypt;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::error;
use rocket::post;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::env;

use super::checks::{check_access_token_exists, check_and_update_refresh_token, check_client_exists};
use super::generate_token::generate_token;
#[derive(Deserialize)]

pub struct SignInUser<'a> {
    email: &'a str,
    password: &'a str,
    client_id: &'a str,
}

#[derive(Serialize)]
pub struct SignInResponse {
    action: String,
    access_token: String,
}

#[post("/signin", data = "<sign_in_user>")]
pub async fn sign_in<'a>(sign_in_user: Json<SignInUser<'a>>) -> Json<SignInResponse> {
    let mut conn: diesel_async::AsyncMysqlConnection = establish_connection().await.unwrap();

    let user_exists: Result<User, _> = user
        .filter(email.eq(sign_in_user.email))
        .first::<User>(&mut conn)
        .await;

    match user_exists {
        Ok(user_taken) => {
            let client_exists = check_client_exists(sign_in_user.client_id, &mut conn).await;

            match client_exists {
                Ok(true) => (),
                Ok(false) => {
                    return Json(SignInResponse {
                        action: "Client does not exist".to_string(),
                        access_token: "".to_string(),
                    });
                }
                Err(_) => {
                    return Json(SignInResponse {
                        action: "Error checking client existence".to_string(),
                        access_token: "".to_string(),
                    });
                }
            }

            let exp_access_token: chrono::DateTime<Utc> = Utc::now() + Duration::hours(1);
            let exp_refresh_token: chrono::DateTime<Utc> = Utc::now() + Duration::days(14);
            let key: Result<String, String> = env::var("USER_ENCRYPTION_KEY").map_err(|e| {
                error!("Error: {}", e);
                "Encryption key must be set".to_string()
            });
            let size: Result<String, String> = env::var("ID_SIZE").map_err(|e| {
                error!("Error: {}", e);
                "Size must be set".to_string()
            });
            let secret: Result<String, String> = env::var("ROCKET_SECRET").map_err(|e| {
                error!("Error: {}", e);
                "Encryption key must be set".to_string()
            });
            let user_key = env::var("USER_ENCRYPTION_KEY").map_err(|e| {
                error!("Error: {}", e);
                "Encryption key must be set".to_string()
            });

            let access_token_exists =
                check_access_token_exists(sign_in_user.client_id, &user_taken.id, &mut conn).await;

            match access_token_exists {
                Ok(present_access_token) => {
                    let secret_str: &String = secret.as_ref().unwrap();

                    let result_check_and_refresh_update = check_and_update_refresh_token(
                        &sign_in_user.client_id,
                        &user_taken.id,
                        &mut conn,
                        &secret_str,
                        &user_taken.email,
                        &user_taken.tenant_id,
                        exp_refresh_token.timestamp() as usize,
                        &user_key.unwrap(),
                    ).await;

                    match result_check_and_refresh_update {
                        Ok(_) => (),
                        Err(e) => {
                            return Json(SignInResponse {
                                action: e.to_string(),
                                access_token: "".to_string(),
                            })
                        }
                    }

                    let access_token: String = generate_token(
                        secret_str,
                        &user_taken.id,
                        &user_taken.email,
                        &user_taken.tenant_id,
                        exp_access_token.timestamp() as usize,
                    );

                    let update_access_token = diesel::update(crate::schema::access_token::table)
                        .filter(
                            crate::schema::access_token::id.eq(&present_access_token.unwrap().id),
                        )
                        .set((
                            crate::schema::access_token::token.eq(&access_token),
                            crate::schema::access_token::expires_at
                                .eq(exp_access_token.naive_utc()),
                        ))
                        .execute(&mut conn)
                        .await
                        .map_err(|e| {
                            error!("Error updating access token: {}", e);
                            "Error updating access token".to_string()
                        });

                    match update_access_token {
                        Ok(_) => (),
                        Err(e) => {
                            return Json(SignInResponse {
                                action: e,
                                access_token: "".to_string(),
                            })
                        }
                    }

                    return Json(SignInResponse {
                        action: "Sign In".to_string(),
                        access_token,
                    });
                }
                Err(_) => {
                    println!("No access token found thus moving forward to create one!");
                }
            }

            let secret_str: &String = secret.as_ref().unwrap();

            let access_token: String = generate_token(
                secret_str,
                &user_taken.id,
                &user_taken.email,
                &user_taken.tenant_id,
                exp_access_token.timestamp() as usize,
            );

            let refresh_token: String = generate_token(
                secret_str,
                &user_taken.id,
                &user_taken.email,
                &user_taken.tenant_id,
                exp_refresh_token.timestamp() as usize,
            );

            let rand_hash_access_token: String =
                generate_random_hash_function(size.clone().unwrap().parse().unwrap());

            let rand_hash_refresh_token: String =
                generate_random_hash_function(size.unwrap().parse().unwrap());

            let refresh_token_hash: String = encrypt(&refresh_token, &key.unwrap(), 16);

            let new_access_token: NewAccessToken = NewAccessToken {
                id: &rand_hash_access_token,
                user_id: &user_taken.id,
                client_id: sign_in_user.client_id,
                token: &access_token,
                expires_at: exp_access_token.naive_utc(),
            };

            let new_refresh_token: NewRefreshToken = NewRefreshToken {
                id: &rand_hash_refresh_token,
                user_id: &user_taken.id,
                client_id: sign_in_user.client_id,
                token: &refresh_token_hash,
                expires_at: exp_refresh_token.naive_utc(),
            };

            let insert_access_token = diesel::insert_into(crate::schema::access_token::table)
                .values(&new_access_token)
                .execute(&mut conn)
                .await
                .map_err(|e| {
                    error!("Error saving new access token: {}", e);
                    "Error saving new access token".to_string()
                });

            let insert_refresh_token = diesel::insert_into(crate::schema::refresh_token::table)
                .values(&new_refresh_token)
                .execute(&mut conn)
                .await
                .map_err(|e| {
                    error!("Error saving new refresh token: {}", e);
                    "Error saving new refresh token".to_string()
                });

            match insert_access_token {
                Ok(_) => (),
                Err(e) => {
                    return Json(SignInResponse {
                        action: e,
                        access_token: "".to_string(),
                    })
                }
            }

            match insert_refresh_token {
                Ok(_) => (),
                Err(e) => {
                    return Json(SignInResponse {
                        action: e,
                        access_token: "".to_string(),
                    })
                }
            }

            return Json(SignInResponse {
                action: "Sign In".to_string(),
                access_token,
            });
        }
        Err(_) => {
            return Json(SignInResponse {
                action: "Sign In Failed".to_string(),
                access_token: "".to_string(),
            });
        }
    }
}
