use super::models::CreateUserResponse;
use super::models::NewUserCreate;
use crate::models::user::User;
use crate::models::{tenant::Tenant, user::NewUser};
use crate::schema::tenant::dsl::{id, tenant};
use crate::schema::user::dsl::{email, tenant_id, user, user_name};
use crate::utils::config::AppConfig;
use crate::utils::connect_sql::establish_connection;
use crate::utils::generate_random_hash::generate_random_hash_function;
use crate::utils::generate_short_hash::encrypt;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::error;
use rocket::http::Status;
use rocket::post;
use rocket::response::status;
use rocket::serde::json::Json;

#[post("/createUser", data = "<new_user_create>")]
pub async fn create_user(
    new_user_create: Json<NewUserCreate<'_>>,
) -> Result<Json<CreateUserResponse>, status::Custom<&str>> {
    let required_vars: Vec<&str> = vec!["ID_SIZE", "USER_ENCRYPTION_KEY"];
    if !AppConfig::check_vars(required_vars) {
        return Err(status::Custom(
            Status::InternalServerError,
            "Required environment variable(s) are not set",
        ));
    }

    let connection: &mut diesel_async::AsyncMysqlConnection =
        &mut establish_connection().await.unwrap();

    let tenant_exists: Result<Tenant, diesel::result::Error> = tenant
        .filter(id.eq(new_user_create.tenant_id))
        .first(connection)
        .await;

    match tenant_exists {
        Ok(_) => {
            let size: String = AppConfig::get_var("ID_SIZE");

            let rand_hash: String = generate_random_hash_function(size.parse().unwrap());

            let key: String = AppConfig::get_var("USER_ENCRYPTION_KEY");

            let password_hash: String = encrypt(new_user_create.password, key.as_str(), 16);

            let new_user: NewUser<'_> = NewUser {
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
                Ok(_) => Ok(Json(CreateUserResponse {
                    action: "User already exists!".to_string(),
                })),
                Err(diesel::result::Error::NotFound) => {
                    let insert_result: Result<usize, String> = diesel::insert_into(user)
                        .values(&new_user)
                        .execute(connection)
                        .await
                        .map_err(|e| {
                            error!("Error saving new user: {}", e);
                            "Error saving new user".to_string()
                        });

                    match insert_result {
                        Ok(_) => Ok(Json(CreateUserResponse {
                            action: "User created successfully!".to_string(),
                        })),
                        Err(e) => {
                            error!("Error saving new user: {}", e);
                            Err(status::Custom(
                                rocket::http::Status::InternalServerError,
                                "Error saving new user",
                            ))
                        }
                    }
                }
                Err(e) => {
                    error!("Error checking user existence: {}", e);
                    Err(status::Custom(
                        rocket::http::Status::InternalServerError,
                        "Error checking user existence",
                    ))
                }
            }
        }
        Err(e) => {
            error!("Error checking tenant existence: {}", e);
            Err(status::Custom(
                rocket::http::Status::NotFound,
                "Error: Tenant does not exist",
            ))
        }
    }
}
