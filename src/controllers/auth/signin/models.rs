use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignInUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub client_id: &'a str,
}

#[derive(Serialize)]
pub struct SignInResponse {
    pub action: String,
    pub access_token: String,
}

pub struct CheckTokenResponse {
    pub success: bool,
    pub token: String,
}
