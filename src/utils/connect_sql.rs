use crate::utils::config::AppConfig;
use diesel_async::{AsyncConnection, AsyncMysqlConnection};
use rocket::error;

pub async fn establish_connection() -> Result<AsyncMysqlConnection, String> {
    let required_vars = vec!["DATABASE_URL"];
    if !AppConfig::check_vars(required_vars) {
        panic!("Required environment variables are not set");
    }

    let database_url = AppConfig::get_var("DATABASE_URL");

    let sql_connection: Result<AsyncMysqlConnection, String> =
        AsyncMysqlConnection::establish(&database_url)
            .await
            .map_err(|e: diesel::ConnectionError| {
                error!("Error: {}", e);
                "Error connecting database!".to_string()
            });

    sql_connection
}
