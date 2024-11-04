use super::checks::{check_and_process_tokens, check_client_exists, check_pass};
use super::generate_token::generate_token;
use crate::models::access_token::NewAccessToken;
use crate::models::refresh_token::NewRefreshToken;
use crate::models::user::User;
use crate::schema::user::dsl::{email, user};
use crate::utils::config::AppConfig;
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
    let required_vars = vec!["USER_ENCRYPTION_KEY", "ID_SIZE", "ROCKET_SECRET"];
    if !AppConfig::check_vars(required_vars) {
        panic!("Required environment variables are not set");
    }

    let mut conn: diesel_async::AsyncMysqlConnection = establish_connection().await.unwrap();

    let user_key = AppConfig::get_var("USER_ENCRYPTION_KEY");
    let size = AppConfig::get_var("ID_SIZE");
    let secret = AppConfig::get_var("ROCKET_SECRET");

    let user_exists: Result<User, _> = user
        .filter(email.eq(sign_in_user.email))
        .first::<User>(&mut conn)
        .await;

    match user_exists {
        Ok(user_taken) => {
            let pass_match = check_pass(
                sign_in_user.password,
                &user_taken.password,
                &user_key.clone(),
            );

            match pass_match {
                Ok(true) => (),
                Ok(false) => {
                    return Json(SignInResponse {
                        action: "Password does not match".to_string(),
                        access_token: "".to_string(),
                    });
                }
                Err(e) => {
                    return Json(SignInResponse {
                        action: e.to_string(),
                        access_token: "".to_string(),
                    });
                }
            }

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

            let exp_access_token: chrono::DateTime<Utc> = Utc::now() + Duration::hours(4);
            let exp_refresh_token: chrono::DateTime<Utc> = Utc::now() + Duration::days(14);

            let access_token_exists_proccessed = check_and_process_tokens(
                &mut conn,
                &secret.clone(),
                &user_key.clone(),
                sign_in_user.client_id,
                &user_taken.id,
                &user_taken.email,
                &user_taken.tenant_id,
                exp_access_token.timestamp() as usize,
                exp_refresh_token.timestamp() as usize,
            )
            .await;

            match access_token_exists_proccessed {
                Ok(_) => (),
                Err(e) => {
                    return Json(SignInResponse {
                        action: e.to_string(),
                        access_token: "".to_string(),
                    });
                }
            }

            let secret_str: &String = &secret;

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
                generate_random_hash_function(size.clone().parse().unwrap());

            let rand_hash_refresh_token: String =
                generate_random_hash_function(size.parse().unwrap());

            let refresh_token_hash: String = encrypt(&refresh_token, &user_key, 16);

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

            match insert_access_token {
                Ok(_) => (),
                Err(e) => {
                    return Json(SignInResponse {
                        action: e,
                        access_token: "".to_string(),
                    })
                }
            }

            let insert_refresh_token = diesel::insert_into(crate::schema::refresh_token::table)
                .values(&new_refresh_token)
                .execute(&mut conn)
                .await
                .map_err(|e| {
                    error!("Error saving new refresh token: {}", e);
                    "Error saving new refresh token".to_string()
                });

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
        Err(e) => {
            return Json(SignInResponse {
                action: e.to_string(),
                access_token: "".to_string(),
            });
        }
    }
}
