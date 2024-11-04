use crate::schema::client::dsl::client;
use crate::schema::tenant::dsl::{id, tenant};
use crate::utils::config::AppConfig;
use crate::utils::generate_random_hash::generate_random_hash_function;
use crate::utils::generate_short_hash::encrypt;
use crate::{
    models::{client::NewClient, tenant::Tenant},
    utils::connect_sql::establish_connection,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::error;
use rocket::post;
use rocket::serde::json::Json;

#[derive(serde::Deserialize)]
pub struct NewClientCreate<'a> {
    name: &'a str,
    tenant_id: &'a str,
    client_secret: &'a str,
    redirect_uri: &'a str,
}

#[derive(serde::Serialize)]
pub struct CreateClientResponse {
    action: String,
    tenant_key_refresher_hash: String,
}

#[post("/createClient", data = "<new_client_create>")]
pub async fn create_client(
    new_client_create: Json<NewClientCreate<'_>>,
) -> Json<CreateClientResponse> {
    let required_vars = vec!["ID_SIZE", "CLIENT_ENCRYPTION_KEY"];
    if !AppConfig::check_vars(required_vars) {
        panic!("Required environment variables are not set");
    }

    let connection: &mut diesel_async::AsyncMysqlConnection =
        &mut establish_connection().await.unwrap();

    let tenant_exists: Result<Tenant, diesel::result::Error> = tenant
        .filter(id.eq(new_client_create.tenant_id))
        .first(connection)
        .await;

    match tenant_exists {
        Ok(_) => {
            let size = AppConfig::get_var("ID_SIZE");
            let rand_hash: String = generate_random_hash_function(size.parse().unwrap());

            let new_client: NewClient<'_> = NewClient {
                id: rand_hash.as_str(),
                tenant_id: new_client_create.tenant_id,
                name: new_client_create.name,
                client_secret: new_client_create.client_secret,
                redirect_uri: new_client_create.redirect_uri,
            };

            let insert_result = diesel::insert_into(client)
                .values(&new_client)
                .execute(connection)
                .await
                .map_err(|e| {
                    error!("Error saving new client: {}", e);
                    "Error saving new client".to_string()
                });

            match insert_result {
                Ok(_) => (),
                Err(e) => {
                    return Json(CreateClientResponse {
                        action: e,
                        tenant_key_refresher_hash: "".to_string(),
                    })
                }
            }

            let key = AppConfig::get_var("CLIENT_ENCRYPTION_KEY");

            let encrypted_text: String = encrypt(rand_hash.as_str(), key.as_str(), 16);

            Json(CreateClientResponse {
                action: "Created Client successfully!".to_string(),
                tenant_key_refresher_hash: encrypted_text,
            })
        }
        Err(e) => {
            return Json(CreateClientResponse {
                action: format!("Error: Tenant does not exist: {}", e),
                tenant_key_refresher_hash: "".to_string(),
            })
        }
    }
}
