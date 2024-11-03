use crate::models::access_token::AccessToken;
use crate::models::client::Client;
use crate::models::refresh_token::RefreshToken;
use crate::schema::access_token::dsl::{
    access_token, client_id as access_token_client_id, user_id as access_token_user_id,
};
use crate::schema::client::dsl::{client, id};
use crate::schema::refresh_token::dsl::{
    client_id as refresh_token_client_id, refresh_token, user_id as refresh_token_user_id,
};
use crate::utils::generate_short_hash::encrypt;
use chrono::Utc;
use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::RunQueryDsl;

use super::generate_token::generate_token;

pub async fn check_access_token_exists(
    provided_client_id: &str,
    provided_user_id: &str,
    conn: &mut diesel_async::AsyncMysqlConnection,
) -> Result<Option<AccessToken>, Error> {
    let access_token_exists = access_token
        .filter(access_token_client_id.eq(provided_client_id))
        .filter(access_token_user_id.eq(provided_user_id))
        .first::<AccessToken>(conn)
        .await;

    match access_token_exists {
        Ok(token) => Ok(Some(token)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub async fn check_client_exists(
    provided_client_id: &str,
    conn: &mut diesel_async::AsyncMysqlConnection,
) -> Result<bool, Error> {
    let client_exists = client
        .filter(id.eq(provided_client_id))
        .first::<Client>(conn)
        .await;

    match client_exists {
        Ok(_) => Ok(true),
        Err(diesel::result::Error::NotFound) => Ok(false),
        Err(e) => Err(e),
    }
}

pub async fn check_and_update_refresh_token(
    provided_client_id: &str,
    provided_user_id: &str,
    conn: &mut diesel_async::AsyncMysqlConnection,
    secret: &str,
    provided_user_email_for_refresh: &str,
    provided_tenant_id_for_refresh: &str,
    exp: usize,
    encrypt_key: &str,
) -> Result<(), Error> {
    let refresh_token_exists = refresh_token
        .filter(refresh_token_client_id.eq(provided_client_id))
        .filter(refresh_token_user_id.eq(provided_user_id))
        .first::<RefreshToken>(conn)
        .await;

    match refresh_token_exists {
        Ok(token) => {
            let new_refresh_token: String = generate_token(
                secret,
                &provided_user_id,
                &provided_user_email_for_refresh,
                provided_tenant_id_for_refresh,
                exp,
            );

            let encrypted_token = encrypt(&new_refresh_token, encrypt_key, 16);

            let current: chrono::NaiveDateTime = Utc::now().naive_utc();
            if current > token.expires_at {
                diesel::update(refresh_token.find(token.id))
                    .set(crate::schema::refresh_token::dsl::token.eq(encrypted_token))
                    .execute(conn)
                    .await?;
            }

            Ok(())
        }
        Err(e) => Err(e),
    }
}
