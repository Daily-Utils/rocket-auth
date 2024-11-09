use crate::schema::authorization_code;
use crate::schema::client::dsl::client;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(client))]
#[diesel(table_name=authorization_code)]
pub struct AuthorizationCode {
    pub id: String,
    pub client_id: String,
    pub user_id: String,
    pub code: String,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=authorization_code)]
pub struct NewAuthorizationCode<'a> {
    pub id: &'a str,
    pub client_id: &'a str,
    pub user_id: &'a str,
    pub code: &'a str,
}
