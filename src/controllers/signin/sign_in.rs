use chrono::{Duration, Utc};
use rocket::post;
use rocket::serde::json::Json;
use std::env;
use crate::models::user::User;
use crate::models::access_token::NewAccessToken;
use crate::models::refresh_token::NewRefreshToken;
use crate::utils::generate_random_hash::generate_random_hash_function;
use crate::utils::generate_short_hash::encrypt;
use crate::utils::{connect_sql::establish_connection, jwt::{sign_jwt, Claims}};
use crate::schema::user::dsl::{user, email};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use rocket::error;
use crate::schema::client::dsl::{client, id};
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
pub async fn sign_in(sign_in_user: Json<SignInUser<'_>>) -> Json<SignInResponse> {
    let mut conn: diesel_async::AsyncMysqlConnection = establish_connection().await.unwrap();

    let user_exists: Result<User, _> = user.filter(email.eq(sign_in_user.email)).first::<User>(&mut conn).await;

    match user_exists {
        Ok(user_taken) => {
            // check for client_id in the database
            let client_exists: Result<crate::models::client::Client, _> = client.filter(id.eq(sign_in_user.client_id)).first::<crate::models::client::Client>(&mut conn).await;

            match client_exists {
                Ok(_) => (),
                Err(_) => {
                    return Json(SignInResponse {
                        action: "Client does not exist".to_string(),
                        access_token: "".to_string(),
                    });
                }
            }


            let exp_access_token = Utc::now() + Duration::hours(1);
            let exp_refresh_token = Utc::now() + Duration::days(14);
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

            // TODO: check for access token and refresh token in the database

            let secret_str: &String = secret.as_ref().unwrap();

            let claims: Claims = Claims {
                sub: user_taken.id.clone(),
                email: user_taken.email.clone(),
                tenant_id: user_taken.tenant_id.clone(),
                exp: exp_access_token.timestamp() as usize,
            };

            let access_token: String = sign_jwt(secret_str, claims).unwrap();

            let refresh_claims: Claims = Claims {
                sub: user_taken.id.clone(),
                email: user_taken.email.clone(),
                tenant_id: user_taken.tenant_id.clone(),
                exp: exp_refresh_token.timestamp() as usize,
            };

            let refresh_token: String = sign_jwt(secret_str, refresh_claims).unwrap();

            let rand_hash_access_token: String = generate_random_hash_function(size.clone().unwrap().parse().unwrap());

            let rand_hash_refresh_token: String = generate_random_hash_function(size.unwrap().parse().unwrap());

            let refresh_token_hash: String = encrypt(&refresh_token, &key.unwrap(), 16);

            let new_access_token: NewAccessToken = NewAccessToken {
                id: &rand_hash_access_token,
                user_id: &user_taken.id,
                client_id: sign_in_user.client_id,
                token: &access_token
            };

            let new_refresh_token: NewRefreshToken = NewRefreshToken {
                id: &rand_hash_refresh_token,
                user_id: &user_taken.id,
                client_id: sign_in_user.client_id,
                token: &refresh_token_hash
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
                Err(e) => return Json(SignInResponse {
                    action: e,
                    access_token: "".to_string(),
                }),
            }

            match insert_refresh_token {
                Ok(_) => (),
                Err(e) => return Json(SignInResponse {
                    action: e,
                    access_token: "".to_string(),
                }),
            }

            return Json(SignInResponse {
                action: "Sign In".to_string(),
                access_token,
            });
        }
        Err(_) => {
            return Json(SignInResponse {
                action: "Sign In Failed".to_string(),
                access_token: "".to_string()
            });
        }
    }
}