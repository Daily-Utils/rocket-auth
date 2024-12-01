#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use crate::utils::jwt::{sign_jwt, verify_jwt, Claims};

    #[test]
    fn test_sign_jwt() {
        let current: chrono::DateTime<Utc> = Utc::now();
        let current_plus_4_hr = current + Duration::hours(4);

        let secret = "my_secret_key";
        let claims = Claims {
            sub: "user_id".to_string(),
            email: "user@example.com".to_string(),
            tenant_id: "tenant_id".to_string(),
            exp: current_plus_4_hr.timestamp() as usize,
        };

        let token = sign_jwt(secret, claims.clone()).expect("Failed to sign JWT");
        assert!(!token.is_empty(), "Token should not be empty");

        let verified_claims = verify_jwt(secret, &token).expect("Failed to verify JWT");
        assert_eq!(verified_claims.sub, claims.sub);
        assert_eq!(verified_claims.email, claims.email);
        assert_eq!(verified_claims.tenant_id, claims.tenant_id);
        assert_eq!(verified_claims.exp, claims.exp);
    }

    #[test]
    fn test_verify_jwt_invalid_token() {
        let secret = "my_secret_key";
        let invalid_token = "invalid.token.string";

        let result = verify_jwt(secret, invalid_token);
        assert!(
            result.is_err(),
            "Verification should fail for an invalid token"
        );
    }
}
