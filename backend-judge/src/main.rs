use dotenvy::dotenv;
use rocket::{get, routes};

pub mod database;
pub mod utils;
mod routes{pub mod users; pub mod contests;}
use crate::routes::users::{create, index, login};
pub mod models;
pub mod schema;
use std::env;
pub mod responses;
pub mod auth;


#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {


    let rocket = rocket::build().mount("/", routes![index, create, login]);
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJlbWFpbCI6ImhlbGxvIiwiZXhwIjoxNzE3OTM3ODIyfQ.0XR-vScWnHLJ1ui6izzNh_50pzF-VsChhtZYf1lUmtpw1YKt_rTvRJoymYbhEAgBQcbso1S2lS2bJi4Tq04yhQ";

    Ok(rocket.into())
}
