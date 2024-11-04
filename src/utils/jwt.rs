use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use rocket::serde::json::serde_json;
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use std::{collections::BTreeMap, error::Error};

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub tenant_id: String,
    pub exp: usize,
}

pub fn sign_jwt(secret: &str, claims: Claims) -> Result<String, Box<dyn Error>> {
    let key: Hmac<Sha384> = Hmac::new_from_slice(secret.as_bytes())?;

    let header: Header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };

    let token: Token<Header, Claims, jwt::Unsigned> = Token::new(header, claims);

    let jwt: Token<Header, Claims, jwt::token::Signed> = token.sign_with_key(&key)?;

    Ok(jwt.as_str().to_string())
}

pub fn verify_jwt(secret: &str, token_str: &str) -> Result<Claims, Box<dyn Error>> {
    let key: Hmac<Sha384> = Hmac::new_from_slice(secret.as_bytes())?;

    let token: Token<Header, BTreeMap<String, serde_json::Value>, _> = Token::parse_unverified(token_str)?;
    let verified_token: Token<Header, BTreeMap<String, serde_json::Value>, jwt::Verified> =
        token.verify_with_key(&key)?;

    let claims_map: &BTreeMap<String, serde_json::Value> = verified_token.claims();
    let claims_json: String = serde_json::to_string(claims_map)?;
    let mut claims: Claims = serde_json::from_str(&claims_json)?;

    if let Some(exp_value) = claims_map.get("exp") {
        if let Some(exp) = exp_value.as_u64() {
            claims.exp = exp as usize;
        }
    }

    Ok(claims)
}
