// src/db/connection.rs

use std::env; // To manage environment variables for database connection

pub fn get_connection() -> Result<String, &'static str> {
    // Simulate getting a connection from environment variables
    if env::var("TEST_DATABASE_URL").is_ok() {
        Ok("Connected to test database".to_string())
    } else {
        Err("Database URL not set")
    }
}

// src/tests/e2e/db_tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_database_connection_e2e() {
        // Ensure the DATABASE_URL is set for the test
        env::set_var("TEST_DATABASE_URL", "test_database_url");

        let result = get_connection();
        assert!(result.is_ok(), "Failed to get a database connection");
        assert_eq!(result.unwrap(), "Connected to test database");

        // Clean up (optional)
        env::remove_var("TEST_DATABASE_URL");    }
}