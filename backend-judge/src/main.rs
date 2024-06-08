use rocket::{get, routes};

pub mod database;
pub mod utils;
mod routes{pub mod users;}
use crate::routes::users::{create, index, login};
pub mod models;
pub mod schema;

pub mod responses;
#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, create, login]);

    Ok(rocket.into())
}
