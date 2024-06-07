use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket::serde::{Deserialize, Serialize};
use crate::schema::users;

// This struct represents a row in the `users` table
#[derive(Queryable,Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub verified: Option<bool>,
}

// This struct is used for inserting a new row into the `users` table
#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub verified: Option<bool>,
}
