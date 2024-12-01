#[cfg(test)]
mod tests {
    use crate::rocket;
    use rocket::{http::Status, serde::json::json};

    #[rocket::async_test]
    async fn create_tenant() {
        use rocket::local::asynchronous::Client;
        let tenant_create_payload = json!({
            "name": "app"
        });
        let client = Client::tracked(rocket().await).await.unwrap();
        let req_for_tenant = client
            .post("/api/tenant/createTenant")
            .json(&tenant_create_payload)
            .dispatch()
            .await;
        assert_eq!(req_for_tenant.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn failure_create_tenant() {
        use rocket::local::asynchronous::Client;
        let tenant_create_payload = json!({
            "namee": "app"
        });
        let client = Client::tracked(rocket().await).await.unwrap();
        let req_for_tenant = client
            .post("/api/tenant/createTenant")
            .json(&tenant_create_payload)
            .dispatch()
            .await;
        assert_eq!(req_for_tenant.status(), rocket::http::Status { code: 422 });
    }
}
