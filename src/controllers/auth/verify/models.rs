#[derive(serde::Deserialize, serde::Serialize)]
pub struct VerifyAuthRoles {
    pub roles: Vec<String>,
    pub is_jwt_valid: bool,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct VerificationResponse {
    pub message: String,
    pub status: usize,
}
