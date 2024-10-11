use crate::schema::user::dsl::user;
use crate::schema::client::dsl::client;
use diesel::prelude::*;
use crate::schema::refresh_token;

#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(belongs_to(user))]
#[diesel(belongs_to(client))]
#[diesel(table_name = refresh_token)]
pub struct RefreshToken {
    pub id: String,
    pub user_id: String,
    pub client_id: String,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=refresh_token)]
pub struct NewRefreshToken<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub client_id: &'a str,
    pub token: &'a str
}
