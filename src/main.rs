mod controllers;
mod models;
mod schema;
mod test;
mod utils;
mod middlewares;

use crate::controllers::auth::signin::sign_in::sign_in;
use crate::controllers::auth::signup::create_user::create_user;
use crate::controllers::client::create_client::create_client;
use crate::controllers::tenant::refresh_tenant_key::refresh_tenant;
use crate::utils::config::AppConfig;
use crate::utils::connect_sql::establish_connection;
use controllers::tenant::create_tenant::create_tenant;
use rocket::{get, launch, routes};
use utils::config::Config;

#[get("/")]
fn hello() -> String {
    format!("Server is running!!!!!")
}

#[launch]
async fn rocket() -> _ {
    AppConfig::load_env();
    let _ = establish_connection().await.unwrap();

    let rocker_build: rocket::Rocket<rocket::Build> = rocket::build();

    rocker_build
        .mount("/", routes![hello])
        .mount("/api/tenant", routes![create_tenant, refresh_tenant])
        .mount("/api/client", routes![create_client])
        .mount("/api/user", routes![create_user, sign_in])
}
