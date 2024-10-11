use diesel_async::{AsyncConnection, AsyncMysqlConnection};
use dotenvy::dotenv;
use std::env;
use rocket::error;

pub async fn establish_connection() -> Result<AsyncMysqlConnection, String> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").map_err(|e|{
        error!("Error: {}", e);
        "Database url must be set".to_string()
    });

    let sql_connection: Result<AsyncMysqlConnection, String> = AsyncMysqlConnection::establish(&database_url.unwrap()).await.map_err(|e: diesel::ConnectionError|{
        error!("Error: {}", e);
        "Error connecting database!".to_string()
    });

    sql_connection
}