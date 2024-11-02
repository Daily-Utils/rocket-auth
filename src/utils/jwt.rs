use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha384;
use serde::{Serialize, Deserialize};
use rocket::serde::json::serde_json;
use std::{collections::BTreeMap, error::Error};
use rocket::error;
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    email: String,
    tenant_id: String,
    exp: usize
}

pub fn sign_jwt(claims: Claims) -> Result<String, Box<dyn Error>> {
    let rocker_secret: Result<String, String> = env::var("ROCKET_SECRET").map_err(|e: env::VarError| {
        error!("Error: {}", e);
        "Rocket secret must be set".to_string()
    });

    let secret: &String = rocker_secret.as_ref().unwrap();

    let key: Hmac<Sha384> = Hmac::new_from_slice(secret.as_bytes())?;


    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };

    let token: Token<Header, Claims, jwt::Unsigned> = Token::new(header, claims);

    let jwt: Token<Header, Claims, jwt::token::Signed> = token.sign_with_key(&key)?;

    Ok(jwt.as_str().to_string())
}

pub fn verify_jwt(token_str: &str) -> Result<Claims, Box<dyn Error>>{
    let rocker_secret: Result<String, String> = env::var("ROCKET_SECRET").map_err(|e: env::VarError| {
        error!("Error: {}", e);
        "Rocket secret must be set".to_string()
    });

    let secret: &String = rocker_secret.as_ref().unwrap();

    let key: Hmac<Sha384> = Hmac::new_from_slice(secret.as_bytes())?;

    let token: Token<Header, BTreeMap<String, String>, _> = Token::parse_unverified(token_str)?;
    let verified_token: Token<Header, BTreeMap<String, String>, jwt::Verified> = token.verify_with_key(&key)?;

    let claims_map: &BTreeMap<String, String> = verified_token.claims();
    let claims_json = serde_json::to_string(claims_map)?;
    let claims: Claims = serde_json::from_str(&claims_json)?;

    Ok(claims)
}