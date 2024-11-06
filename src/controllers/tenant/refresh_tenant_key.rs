use crate::schema::client::dsl::client;
use crate::utils::config::AppConfig;
use crate::utils::connect_sql::establish_connection;
use crate::utils::generate_short_hash::decrypt;
use crate::utils::generate_short_hash::encrypt;
use diesel::query_dsl::methods::FilterDsl;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use rocket::error;
use rocket::http::Status;
use rocket::post;
use rocket::response::status;
use rocket::serde::json::Json;
use std::error::Error;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RefreshRequest<'a> {
    client_key: &'a str,
}

#[derive(serde::Serialize)]
pub struct RefreshTenantKeyResponse {
    action: String,
    tenant_key: String,
}

#[post("/refreshTenantKey", data = "<refresh_token>")]
pub async fn refresh_tenant<'a>(
    refresh_token: Json<RefreshRequest<'a>>,
) -> Result<Json<RefreshTenantKeyResponse>, status::Custom<&str>> {
    let required_vars = vec!["ENCRYPTION_KEY"];
    if !AppConfig::check_vars(required_vars) {
        return Err(status::Custom(
            Status::InternalServerError,
            "Required environment variable(s) are not set",
        ));
    }

    let connection: &mut diesel_async::AsyncMysqlConnection =
        &mut establish_connection().await.unwrap();

    let key = AppConfig::get_var("ENCRYPTION_KEY");

    let key_str: &String = &key;
    let decrypted_text: Result<String, Box<dyn Error>> =
        decrypt(refresh_token.client_key, key_str.as_str());

    let client_query: Result<crate::models::client::Client, _> = client
        .filter(crate::schema::client::dsl::id.eq(decrypted_text.unwrap()))
        .first::<crate::models::client::Client>(connection)
        .await
        .map_err(|e| {
            error!("Error saving new client: {}", e);
            "Error saving new client".to_string()
        });

    match client_query {
        Ok(client_query) => {
            let encrypted_text: String =
                encrypt(client_query.tenant_id.as_str(), key_str.as_str(), 16);

            Ok(Json(RefreshTenantKeyResponse {
                action: "Tenant key refreshed".to_string(),
                tenant_key: encrypted_text,
            }))
        }
        Err(e) => {
            let error_message = format!("Error saving new client: {}", e);
            Err(status::Custom(
                Status::InternalServerError,
                Box::leak(error_message.into_boxed_str()),
            ))
        }
    }
}
