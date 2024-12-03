use crate::controllers::auth::verify::models::VerifyAuthRoles;
use crate::utils::config::AppConfig;
use crate::utils::jwt::verify_jwt;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for VerifyAuthRoles {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = req.headers().get_one("Authorization");
        let required_vars: Vec<&str> = vec!["ROCKET_SECRET"];
        if !AppConfig::check_vars(required_vars) {
            return Outcome::Error((Status::InternalServerError, ()));
        }
        match token {
            Some(token) if token.starts_with("Bearer ") => {
                let jwt = &token[7..];
                match verify_jwt(&AppConfig::get_var("ROCKET_SECRET"), jwt) {
                    Ok(_) => {
                        return Outcome::Success(VerifyAuthRoles {
                            roles: [].to_vec(),
                            is_jwt_valid: true,
                        })
                    }
                    Err(e) => {
                        println!("Error verifying jwt: {:?}", e);
                        return Outcome::Error((Status::Unauthorized, ()));
                    }
                }
            }
            _ => {
                return Outcome::Error((Status::Unauthorized, ()));
            }
        }
    }
}
