#[derive(serde::Deserialize)]
pub struct NewTenantCreate<'a> {
    pub name: &'a str,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateTenantResponse {
    pub action: String,
    pub tenant_key: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RefreshRequest<'a> {
    pub client_key: &'a str,
}

#[derive(serde::Serialize)]
pub struct RefreshTenantKeyResponse {
    pub action: String,
    pub tenant_key: String,
}
