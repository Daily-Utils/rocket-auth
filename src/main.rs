mod controllers;
mod models;
mod schema;
mod utils;
mod test;

use crate::controllers::client::create_client::create_client;
use crate::controllers::tenant::refresh_tenant_key::refresh_tenant;
use crate::utils::connect_sql::establish_connection;
use controllers::tenant::create_tenant::create_tenant;
use crate::controllers::signup::create_user::create_user;
use rocket::{get, launch, routes};

#[get("/")]
fn hello() -> String {
    format!("Server is runningsss!!!!!")
}

#[launch]
async fn rocket() -> _ {
    let _ = establish_connection().await.unwrap();

    let rocker_build: rocket::Rocket<rocket::Build> = rocket::build();

    rocker_build
        .mount("/", routes![hello])
        .mount("/api/tenant", routes![create_tenant, refresh_tenant])
        .mount("/api/client", routes![create_client])
        .mount("/api/user", routes![create_user])
}
