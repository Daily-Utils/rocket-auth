use crate::controllers::auth::verify::models::VerifyAuthRoles;
use rocket::{post, response::status, serde::json::Json};

use super::models::VerificationResponse;

#[post("/verify")]
pub fn verify(
    _auth: VerifyAuthRoles,
) -> Result<Json<VerificationResponse>, status::Custom<String>> {
    if _auth.is_jwt_valid == true {
        // TODO: Add checks for role
        return Ok(Json(VerificationResponse {
            message: "verification successfull".to_string(),
            status: 200,
        }));
    } else {
        return Err(status::Custom(
            rocket::http::Status::Unauthorized,
            "Invalid JWT".to_string(),
        ));
    }
}
