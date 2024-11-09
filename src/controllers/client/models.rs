#[derive(serde::Deserialize)]
pub struct NewClientCreate<'a> {
    pub name: &'a str,
    pub tenant_id: &'a str,
    pub client_secret: &'a str,
    pub redirect_uri: &'a str,
}

#[derive(serde::Serialize)]
pub struct CreateClientResponse {
    pub action: String,
    pub tenant_key_refresher_hash: String,
}
