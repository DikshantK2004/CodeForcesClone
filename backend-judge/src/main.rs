use dotenvy::dotenv;
use rocket::{get, routes};

pub mod database;
pub mod utils;
mod routes{pub mod users; pub mod contests;}
use crate::routes::users::{create, index, login};
pub mod models;
pub mod schema;
use std::env;
use crate::routes::contests::create_contest;

pub mod responses;
pub mod auth;


#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {


    let rocket = rocket::build().mount("/", routes![index, create, login, create_contest]);

    Ok(rocket.into())
}
