use crate::schema::tenant::dsl::tenant;
use crate::utils::config::AppConfig;
use crate::utils::generate_random_hash::generate_random_hash_function;
use crate::utils::generate_short_hash::encrypt;
use crate::{models::tenant::NewTenant, utils::connect_sql::establish_connection};
use diesel_async::RunQueryDsl;
use rocket::error;
use rocket::http::Status;
use rocket::post;
use rocket::response::status;
use rocket::serde::json::Json;

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
pub async fn create_tenant(
    new_tenant_create: Json<NewTenantCreate<'_>>,
) -> Result<Json<CreateTenantResponse>, status::Custom<&str>> {
    let required_vars = vec!["ID_SIZE", "ENCRYPTION_KEY"];
    if !AppConfig::check_vars(required_vars) {
        return Err(status::Custom(
            Status::InternalServerError,
            "Required environment variable(s) are not set",
        ));
    }

    let connection: &mut diesel_async::AsyncMysqlConnection =
        &mut establish_connection().await.unwrap();

    let size = AppConfig::get_var("ID_SIZE");

    let rand_hash: String = generate_random_hash_function(size.parse().unwrap());

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
        Err(e) => {
            error!("Error saving new tenant: {}", e);
            return Err(status::Custom(
                Status::InternalServerError,
                "Error saving new tenant",
            ))
        }
    }

    let key = AppConfig::get_var("ENCRYPTION_KEY");

    let encrypted_text: String = encrypt(rand_hash.as_str(), key.as_str(), 16);

    Ok(Json(CreateTenantResponse {
        action: "Created Tenant successfully!".to_string(),
        tenant_key: encrypted_text,
    }))
}
