use dotenvy::dotenv;
use rocket::{catchers, get, routes};

pub mod database;
pub mod utils;
mod routes{
    pub mod users;
    pub mod contests;
    pub mod problems;
    pub mod submissions;
}

pub mod submission_utils;
use crate::routes::users::{create, guard, index, login};
use crate::routes::submissions::*;
pub mod models;
pub mod schema;
use crate::routes::contests::{create_contest, update_contest, get_all_contests, get_particular_contest};
use crate::routes::problems::{get_all_problems, get_particular_problem};

pub mod responses;
pub mod auth;
pub mod contest_utils;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, create, login, guard])
        .mount("/contests/", routes![create_contest, update_contest, get_all_contests, get_particular_contest])
        .mount("/problems/", routes![get_all_problems, get_particular_problem])
        .mount("/submit", routes![submit, general_submission_handler, user_submissions, leaderboard, user_contest_submissions, contest_submissions, particular_submission]);

    Ok(rocket.into())
}
