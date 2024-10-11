use rocket::post;
use crate::{models::tenant::NewTenant, utils::connect_sql::establish_connection};
use rocket::serde::json::Json;
use crate::schema::tenant::dsl::tenant;
use diesel_async::RunQueryDsl;
use crate::utils::generate_random_hash::generate_random_hash_function;
use crate::utils::generate_short_hash::encrypt;
use std::env;
use rocket::error;

#[derive(serde::Deserialize)]
pub struct NewTenantCreate<'a> {
    name: &'a str,
}

#[derive(serde::Serialize)]
pub struct CreateTenantResponse {
    action: String,
    tenant_key: String,
}

#[post("/createTenant", data = "<new_tenant_create>")]
pub async fn create_tenant(new_tenant_create: Json<NewTenantCreate<'_>>) -> Json<CreateTenantResponse> {
    let connection: &mut diesel_async::AsyncMysqlConnection = &mut establish_connection().await.unwrap();

    let size: Result<String, String> = env::var("SIZE_LEN_LIMIT_STR").map_err(|e|{
        error!("Error: {}", e);
        "Size must be set".to_string()
    });

    let rand_hash: String = generate_random_hash_function(size.unwrap().parse().unwrap());

    let new_tenant: NewTenant<'_> = NewTenant {
        id: rand_hash.as_str(),
        name: new_tenant_create.name,
    };

    let insert_result = diesel::insert_into(tenant)
        .values(new_tenant)
        .execute(connection)
        .await
        .map_err(|e| {
            error!("Error saving new tenant: {}", e);
            "Error saving new tenant".to_string()
        });

    match insert_result {
        Ok(_) => (),
        Err(e) => return Json(CreateTenantResponse {
            action: e,
            tenant_key: "".to_string(),
        }),
    }

    let key: Result<String, String> = env::var("ENCRYPTION_KEY").map_err(|e| {
        error!("Error: {}", e);
        "Encryption key must be set".to_string()
    });

    let encrypted_text: String = encrypt(rand_hash.as_str(), key.unwrap().as_str(), 16);

    Json(CreateTenantResponse {
        action: "Created Tenant successfully!".to_string(),
        tenant_key: encrypted_text,
    })
}