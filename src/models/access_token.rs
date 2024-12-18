use crate::schema::access_token;
use crate::schema::client::dsl::client;
use crate::schema::user::dsl::user;
use diesel::prelude::*;

#[derive(Queryable, Associations, Identifiable, Selectable, QueryableByName)]
#[diesel(belongs_to(user))]
#[diesel(belongs_to(client))]
#[diesel(table_name=access_token)]
pub struct AccessToken {
    pub id: String,
    pub client_id: String,
    pub user_id: String,
    pub token: String,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=access_token)]
pub struct NewAccessToken<'a> {
    pub id: &'a str,
    pub client_id: &'a str,
    pub user_id: &'a str,
    pub token: &'a str,
    pub expires_at: chrono::NaiveDateTime,
}
