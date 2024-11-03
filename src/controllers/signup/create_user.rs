use crate::models::user::User;
use crate::schema::tenant::dsl::{id, tenant};
use crate::schema::user::dsl::{user, tenant_id, email, user_name};
use crate::utils::connect_sql::establish_connection;
use crate::utils::generate_short_hash::encrypt;
use crate::utils::generate_random_hash::generate_random_hash_function;
use crate::models::{tenant::Tenant, user::NewUser};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::error;
use rocket::post;
use rocket::serde::json::Json;
use std::env;

#[derive(serde::Deserialize)]
pub struct NewUserCreate<'a> {
    tenant_id: &'a str,
    user_name: &'a str,
    email: &'a str,
    password: &'a str,
}

#[derive(serde::Serialize)]
pub struct CreateUserResponse {
    action: String,
}

#[post("/createUser", data = "<new_user_create>")]
pub async fn create_user(
    new_user_create: Json<NewUserCreate<'_>>,
) -> Json<CreateUserResponse> {
    let connection: &mut diesel_async::AsyncMysqlConnection = &mut establish_connection().await.unwrap();

    // Check if the tenant exists
    let tenant_exists: Result<Tenant, diesel::result::Error> = tenant
        .filter(id.eq(new_user_create.tenant_id))
        .first(connection)
        .await;

    match tenant_exists {
        Ok(_) => {
            let size: Result<String, String> = env::var("ID_SIZE").map_err(|e| {
                error!("Error: {}", e);
                "Size must be set".to_string()
            });

            let rand_hash: String = generate_random_hash_function(size.unwrap().parse().unwrap());

            let key: Result<String, String> = env::var("USER_ENCRYPTION_KEY").map_err(|e| {
                error!("Error: {}", e);
                "Encryption key must be set".to_string()
            });

            let password_hash: String = encrypt(new_user_create.password, key.unwrap().as_str(), 16);

            let new_user = NewUser {
                id: &rand_hash,
                tenant_id: new_user_create.tenant_id,
                user_name: new_user_create.user_name,
                email: new_user_create.email,
                password: &password_hash,
            };

            let user_exists: Result<User, diesel::result::Error> = user
                .filter(tenant_id.eq(new_user_create.tenant_id))
                .filter(user_name.eq(new_user_create.user_name))
                .filter(email.eq(new_user_create.email))
                .first(connection)
                .await;

            match user_exists {
                Ok(_) => {
                    Json(CreateUserResponse {
                        action: "User already exists!".to_string(),
                    })
                }
                Err(diesel::result::Error::NotFound) => {
                    let insert_result = diesel::insert_into(user)
                        .values(&new_user)
                        .execute(connection)
                        .await
                        .map_err(|e| {
                            error!("Error saving new user: {}", e);
                            "Error saving new user".to_string()
                        });

                    match insert_result {
                        Ok(_) => Json(CreateUserResponse {
                            action: "User created successfully!".to_string(),
                        }),
                        Err(e) => Json(CreateUserResponse {
                            action: e,
                        }),
                    }
                }
                Err(e) => {
                    Json(CreateUserResponse {
                        action: format!("Error checking user existence: {}", e),
                    })
                }
            }
        }
        Err(e) => Json(CreateUserResponse {
            action: format!("Error: Tenant does not exist: {}", e),
        }),
    }
}