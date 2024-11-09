#[derive(serde::Deserialize)]
pub struct NewUserCreate<'a> {
    pub tenant_id: &'a str,
    pub user_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(serde::Serialize)]
pub struct CreateUserResponse {
    pub action: String,
}
