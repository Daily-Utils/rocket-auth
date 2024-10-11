mod utils;
mod models;
mod schema;
mod controllers;

use crate::utils::connect_sql::establish_connection;
use crate::controllers::tenant:: refresh_tenant_key::refresh_tenant;
use crate::controllers::client::create_client::create_client;
use controllers::tenant::create_tenant::create_tenant;
use rocket::{get, routes, launch};

#[get("/")]
fn hello() -> String {
    format!("Server is running!")
}

#[launch]
async fn rocket() -> _ {
    let _ = establish_connection().await.unwrap();
    
    let rocker_build: rocket::Rocket<rocket::Build> = rocket::build();

    rocker_build
        .mount("/", routes![hello])
        .mount("/api/tenant", routes![create_tenant, refresh_tenant])
        .mount("/api/client", routes![create_client])
}
