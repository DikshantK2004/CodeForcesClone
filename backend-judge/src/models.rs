use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket::data::FromData;
use rocket::serde::{Deserialize, Serialize};
use crate::schema::users;

// This struct represents a row in the `users` table
#[derive(Queryable,Debug, Serialize, Deserialize, Selectable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub verified: bool,
}

// This struct is used for inserting a new row into the `users` table
#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub verified: Option<bool>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
}
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ContestRequest{
//     pub name: String,
//     pub description: String,
//     pub start_date: chrono::Utc::datetime,
//     pub end_date: chrono::Utc::datetime,
//     pub creator_id: i32,
//     pub problems: Vec<ProblemInput>,
// }
//
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ProblemInput{
//     pub name: String,
//     pub content: String,
//     pub test_cases: Vec<TestCase>
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct TestCase{
//     pub input: String,
//     pub output: String
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Contest{
//     pub id: i32,
//     pub name: String,
//     pub description: String,
//     pub start_date: chrono::Utc::datetime,
//     pub end_date: chrono::Utc::datetime,
//     pub creator_id: i32,
//     pub problems: Option<Vec<Problem>>
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Problem{
//     pub id: i32,
//     pub name: String,
//     pub content: String,
//     pub test_cases: Option<Vec<TestCase>>
// }