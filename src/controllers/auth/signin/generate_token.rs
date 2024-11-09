use crate::utils::jwt::{sign_jwt, Claims};

pub fn generate_token(
    secret: &str,
    sub: &str,
    email_user: &str,
    tenant_id: &str,
    exp: usize,
) -> String {
    let claims: Claims = Claims {
        sub: sub.to_string(),
        email: email_user.to_string(),
        tenant_id: tenant_id.to_string(),
        exp: exp,
    };

    let token: String = sign_jwt(secret, claims).unwrap();

    token.to_string()
}
