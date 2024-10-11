use crate::schema::client::dsl::client;
use crate::utils::connect_sql::establish_connection;
use crate::utils::generate_short_hash::decrypt;
use crate::utils::generate_short_hash::encrypt;
use diesel::query_dsl::methods::FilterDsl;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use rocket::error;
use rocket::post;
use rocket::serde::json::Json;
use std::env;
use std::error::Error;

#[derive(serde::Deserialize)]
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
) -> Json<RefreshTenantKeyResponse> {
    let key: Result<String, String> = env::var("ENCRYPTION_KEY").map_err(|e| {
        error!("Error: {}", e);
        "Encryption key must be set".to_string()
    });

    let key_str: &String = key.as_ref().unwrap();
    let decrypted_text: Result<String, Box<dyn Error>> =
        decrypt(refresh_token.client_key, key_str.as_str());

    let client_query: Result<crate::models::client::Client, _> = client
        .filter(crate::schema::client::dsl::id.eq(decrypted_text.unwrap()))
        .first::<crate::models::client::Client>(&mut establish_connection().await)
        .await;

    match client_query {
        Ok(client_query) => {
            let encrypted_text: String = encrypt(
                client_query.tenant_id.as_str(),
                key_str.as_str(),
                16,
            );

            Json(RefreshTenantKeyResponse {
                action: "Tenant key refreshed".to_string(),
                tenant_key: encrypted_text,
            })
        }
        Err(e) => Json(RefreshTenantKeyResponse {
            action: e.to_string(),
            tenant_key: "".to_string(),
        }),
    }
}
