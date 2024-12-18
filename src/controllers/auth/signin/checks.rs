use crate::models::access_token::AccessToken;
use crate::models::client::Client;
use crate::models::refresh_token::RefreshToken;
use crate::schema::access_token::dsl::{
    access_token, client_id as access_token_client_id, expires_at as access_token_expires_at,
    id as access_token_id, token as db_access_token, user_id as access_token_user_id,
};
use crate::schema::client::dsl::{client, id as db_client_id};
use crate::schema::refresh_token::dsl::{
    client_id as refresh_token_client_id, expires_at as refresh_token_expires_at, refresh_token,
    token as db_refresh_token, user_id as refresh_token_user_id,
};
use crate::utils::generate_short_hash::{decrypt, encrypt};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::RunQueryDsl;
use rocket::error;

use super::generate_token::generate_token;
use super::models::CheckTokenResponse;

pub async fn check_and_process_tokens(
    conn: &mut diesel_async::AsyncMysqlConnection,
    secret_str: &str,
    user_key: &str,
    provided_client_id_for_process: &str,
    provided_user_id_for_process: &str,
    provided_tenant_id_for_process: &str,
    provided_email_for_process: &str,
    exp_access_token: usize,
    exp_refresh_token: usize,
) -> Result<CheckTokenResponse, Error> {
    let access_token_exists: Result<AccessToken, _> = access_token
        .filter(access_token_client_id.eq(provided_client_id_for_process))
        .filter(access_token_user_id.eq(provided_user_id_for_process))
        .first::<AccessToken>(conn)
        .await;

    match access_token_exists {
        Ok(present_access_token) => {
            let result_check_and_refresh_update: Result<(), Error> =
                check_and_update_refresh_token(
                    &provided_client_id_for_process,
                    &provided_user_id_for_process,
                    conn,
                    &secret_str,
                    provided_email_for_process,
                    provided_tenant_id_for_process,
                    exp_refresh_token,
                    &user_key,
                )
                .await;

            if let Err(e) = result_check_and_refresh_update {
                return Err(e);
            }

            let new_access_token: String = generate_token(
                secret_str,
                &provided_user_id_for_process,
                &provided_email_for_process,
                &provided_tenant_id_for_process,
                exp_access_token,
            );

            let current: chrono::NaiveDateTime = Utc::now().naive_utc();
            let current_plus_4_hr: chrono::NaiveDateTime = current + Duration::hours(4);

            let update_access_token: Result<usize, Error> =
                diesel::update(crate::schema::access_token::table)
                    .filter(access_token_id.eq(&present_access_token.id))
                    .set((
                        db_access_token.eq(&new_access_token),
                        access_token_expires_at.eq(current_plus_4_hr),
                    ))
                    .execute(conn)
                    .await
                    .map_err(|e| {
                        error!("Error updating access token: {}", e);
                        e
                    });

            if let Err(e) = update_access_token {
                return Err(e);
            }

            return Ok(CheckTokenResponse {
                token: new_access_token,
                success: true,
            });
        }
        Err(_) => {
            println!("No access token found thus moving forward to create one!");
            return Ok(CheckTokenResponse {
                token: "".to_string(),
                success: false,
            });
        }
    }
}

pub async fn check_client_exists(
    provided_client_id: &str,
    conn: &mut diesel_async::AsyncMysqlConnection,
) -> Result<bool, Error> {
    let client_exists: Result<Client, Error> = client
        .filter(db_client_id.eq(provided_client_id))
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
    let refresh_token_exists: Result<RefreshToken, Error> = refresh_token
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

            let encrypted_token: String = encrypt(&new_refresh_token, encrypt_key, 16);

            let current: chrono::NaiveDateTime = Utc::now().naive_utc();
            let one_day_before_expiration: chrono::NaiveDateTime =
                token.expires_at - Duration::days(1);
            let current_plus_14_days: chrono::NaiveDateTime = current + Duration::days(14);

            if current > one_day_before_expiration {
                diesel::update(refresh_token.find(token.id))
                    .set((
                        db_refresh_token.eq(encrypted_token),
                        refresh_token_expires_at.eq(current_plus_14_days),
                    ))
                    .execute(conn)
                    .await?;
            }

            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn check_pass(
    provided_password: &str,
    database_password: &str,
    key: &str,
) -> Result<bool, Error> {
    let decrypted_password = decrypt(database_password, key);

    if decrypted_password.unwrap() == provided_password {
        Ok(true)
    } else {
        Ok(false)
    }
}
