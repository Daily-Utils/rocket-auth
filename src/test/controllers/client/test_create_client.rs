#[cfg(test)]
mod tests {
    use std::env;

    use crate::rocket;
    use crate::{
        controllers::tenant::models::CreateTenantResponse, utils::generate_short_hash::decrypt,
    };
    use rocket::{http::Status, serde::json::json};

    #[rocket::async_test]
    async fn test_create_client() {
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

        let create_tenant_res = req_for_tenant
            .into_json::<CreateTenantResponse>()
            .await
            .unwrap();

        let key = env::var("CLIENT_ENCRYPTION_KEY").unwrap();
        let tenant_id = decrypt(create_tenant_res.tenant_key.as_str(), key.as_str());

        let create_client_payload = json!({
            "name": "client",
            "tenant_id": tenant_id.unwrap(),
            "client_secret": create_tenant_res.tenant_key,
            "redirect_uri": "http://localhost:8000",
        });
        let req_for_client = client
            .post("/api/client/createClient")
            .json(&create_client_payload)
            .dispatch()
            .await;
        assert_eq!(req_for_client.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn test_failure_create_client() {
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

        let create_tenant_res = req_for_tenant
            .into_json::<CreateTenantResponse>()
            .await
            .unwrap();

        let tenant_id = "tenant_id";

        let create_client_payload = json!({
            "name": "client",
            "tenant_id": tenant_id,
            "client_secret": create_tenant_res.tenant_key,
            "redirect_uri": "http://localhost:8000",
        });
        let req_for_client = client
            .post("/api/client/createClient")
            .json(&create_client_payload)
            .dispatch()
            .await;
        assert_eq!(req_for_client.status(), Status::InternalServerError);
    }
}

// pub name: &'a str,
// pub tenant_id: &'a str,
// pub client_secret: &'a str,
// pub redirect_uri: &'a str,
