#[cfg(test)]
mod tests {
    use rocket::local::asynchronous::Client;
    use rocket::http::{Status, Header};
    use rocket::serde::json::json;
    use serde_json::Value;
    use std::env;
    use rocket::error;

    use crate::utils::generate_random_hash::generate_random_hash_function;

    use crate::rocket;

    #[tokio::test]
    async fn test_create_tenant_success() {
        let rocket_instance = rocket().await;

        let client = Client::tracked(rocket_instance).await.expect("valid rocket instance");

        let size: Result<String, String> = env::var("TEST_ID_SIZE").map_err(|e|{
            error!("Error: {}", e);
            "Size must be set".to_string()
        });

        let rand_hash: String = generate_random_hash_function(size.unwrap().parse().unwrap());

        let request_body = json!({
            "id": rand_hash.to_string(),
            "name": "TestTenant"
        });

        let response = client.post("/api/tenant/createTenant")
            .header(Header::new("Content-Type", "application/json"))
            .body(request_body.to_string())
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let response_json: Value = response.into_json().await.expect("valid JSON response");

        assert_eq!(response_json["action"], "Created Tenant successfully!");
        assert!(response_json["tenant_key"].as_str().unwrap().len() > 0, "tenant_key should be present");
    }

    #[tokio::test]
    async fn test_create_tenant_missing_name() {
        let rocket_instance = rocket().await;

        let client = Client::tracked(rocket_instance).await.expect("valid rocket instance");

        let request_body = json!({});

        let response = client.post("/api/tenant/createTenant")
            .header(Header::new("Content-Type", "application/json"))
            .body(request_body.to_string())
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::UnprocessableEntity);

        // let response_json: Value = response.into_json().await.expect("valid JSON response");
        // assert_eq!(response_json["error"], "Missing field 'name'");
    }

    // #[tokio::test]
    // async fn test_create_tenant_missing_env_var() {
    //     env::remove_var("TENANT_ENCRYPTION_KEY");
    //     env::set_var("ID_SIZE", "16");

    //     let rocket_instance = rocket().await;
    //     let client = Client::tracked(rocket_instance).await.expect("valid rocket instance");

    //     let request_body = json!({
    //         "name": "TestTenant"
    //     });

    //     let response = client.post("/api/tenant/createTenant")
    //         .header(Header::new("Content-Type", "application/json"))
    //         .body(request_body.to_string())
    //         .dispatch()
    //         .await;

    //     assert_eq!(response.status(), Status::InternalServerError);

    //     let response_json: Value = response.into_json().await.expect("valid JSON response");
    //     assert_eq!(response_json["error"], "Encryption key must be set");

    //     env::set_var("TENANT_ENCRYPTION_KEY", "some_secret_key");
    // }
}