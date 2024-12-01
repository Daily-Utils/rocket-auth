#[cfg(test)]
mod tests {
    use std::env;

    use crate::{
        controllers::{client::models::CreateClientResponse, tenant::models::CreateTenantResponse},
        rocket,
        utils::generate_short_hash::decrypt,
    };
    use rocket::{http::Status, serde::json::json};

    #[rocket::async_test]
    async fn test_user_signin() {
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
        let tenant_id = decrypt(create_tenant_res.tenant_key.as_str(), key.as_str()).unwrap();
        let create_client_payload = json!({
            "name": "client",
            "tenant_id": tenant_id.clone(),
            "client_secret": create_tenant_res.tenant_key,
            "redirect_uri": "http://localhost:8000",
        });
        let req_for_client = client
            .post("/api/client/createClient")
            .json(&create_client_payload)
            .dispatch()
            .await;
        assert_eq!(req_for_client.status(), Status::Ok);

        let create_user = json!({
            "tenant_id": tenant_id,
            "user_name": "user",
            "email": "user@gmail.com",
            "password": "password"
        });

        let req_for_user = client
            .post("/api/user/createUser")
            .json(&create_user)
            .dispatch()
            .await;

        assert_eq!(req_for_user.status(), Status::Ok);

        let create_client_res = req_for_client
            .into_json::<CreateClientResponse>()
            .await
            .unwrap();

        let client_id = decrypt(
            &create_client_res.tenant_key_refresher_hash.as_str(),
            key.as_str(),
        )
        .unwrap();

        let signin_user = json!({
            "client_id": client_id,
            "email": "user@gmail.com",
            "password": "password"
        });

        let req_for_signin = client
            .post("/api/user/signin")
            .json(&signin_user)
            .dispatch()
            .await;

        assert_eq!(req_for_signin.status(), Status::Ok);
    }

    #[rocket::async_test]
    async fn test_user_signin_wrong_tenant() {
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
        let tenant_id = decrypt(create_tenant_res.tenant_key.as_str(), key.as_str()).unwrap();
        let create_client_payload = json!({
            "name": "client",
            "tenant_id": tenant_id.clone(),
            "client_secret": create_tenant_res.tenant_key,
            "redirect_uri": "http://localhost:8000",
        });
        let req_for_client = client
            .post("/api/client/createClient")
            .json(&create_client_payload)
            .dispatch()
            .await;
        assert_eq!(req_for_client.status(), Status::Ok);

        let create_user = json!({
            "tenant_id": tenant_id,
            "user_name": "user",
            "email": "user@gmail.com",
            "password": "password"
        });

        let req_for_user = client
            .post("/api/user/createUser")
            .json(&create_user)
            .dispatch()
            .await;

        assert_eq!(req_for_user.status(), Status::Ok);

        let create_client_res = req_for_client
            .into_json::<CreateClientResponse>()
            .await
            .unwrap();
        let client_id = decrypt(
            &create_client_res.tenant_key_refresher_hash.as_str(),
            key.as_str(),
        )
        .unwrap();

        let signin_user = json!({
            "client_id": client_id + "wrong",
            "email": "user@gmail.com",
            "password": "password"
        });

        let req_for_signin = client
            .post("/api/user/signin")
            .json(&signin_user)
            .dispatch()
            .await;

        assert_eq!(req_for_signin.status(), rocket::http::Status::BadRequest);
    }

    #[rocket::async_test]
    async fn test_user_signin_wrong_password() {
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
        let tenant_id = decrypt(create_tenant_res.tenant_key.as_str(), key.as_str()).unwrap();
        let create_client_payload = json!({
            "name": "client",
            "tenant_id": tenant_id.clone(),
            "client_secret": create_tenant_res.tenant_key,
            "redirect_uri": "http://localhost:8000",
        });
        let req_for_client = client
            .post("/api/client/createClient")
            .json(&create_client_payload)
            .dispatch()
            .await;
        assert_eq!(req_for_client.status(), Status::Ok);

        let create_user = json!({
            "tenant_id": tenant_id,
            "user_name": "user",
            "email": "user@gmail.com",
            "password": "password"
        });

        let req_for_user = client
            .post("/api/user/createUser")
            .json(&create_user)
            .dispatch()
            .await;

        assert_eq!(req_for_user.status(), Status::Ok);
        let create_client_res = req_for_client
            .into_json::<CreateClientResponse>()
            .await
            .unwrap();
        let client_id = decrypt(
            &create_client_res.tenant_key_refresher_hash.as_str(),
            key.as_str(),
        )
        .unwrap();
        let signin_user = json!({
            "client_id": client_id,
            "email": "user@gmail.com",
            "password": "passworddd"
        });

        let req_for_signin = client
            .post("/api/user/signin")
            .json(&signin_user)
            .dispatch()
            .await;

        assert_eq!(req_for_signin.status(), rocket::http::Status::BadRequest);
    }
}
