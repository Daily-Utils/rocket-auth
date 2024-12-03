pub struct VerifyAuthRoles {
    pub roles: Vec<String>,
    pub is_jwt_valid: bool,
}

pub struct verificationResponse {
    pub message: String,
    pub status: usize,
}
