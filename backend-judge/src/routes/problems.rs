use std::ffi::OsStr;
use std::path::Path;
use chrono::NaiveDateTime;
use rocket::{Data, FromForm, get, post, put, Response, State};
use crate::database::establish_connection;
use crate::models::*;
// import Status
use rocket::http::Status;
use rocket::serde::json::Json;
use diesel::result;
use diesel::result::{DatabaseErrorKind, Error};
use crate::responses::MessageResponse;
use rocket::response::status;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use rocket::data::ToByteUnit;
use rocket::figment::Source::File;
use rocket::form::Form;
use rocket::fs::TempFile;
use crate::utils::*;
use uuid::Uuid;
use crate::models::GeneralContestInfo;

use crate::contest_utils::*;
use crate::schema::contests::dsl::contests;
use crate::schema::problems::dsl::problems;

#[get("/all")]
pub fn get_all_problems() -> (Status, Json<Vec<GeneralProblemInfo>>){
    let connection = &mut establish_connection();
    let all_problems: Vec<GeneralProblemInfo> = problems.select(GeneralProblemInfo::as_select()).load(connection).expect("Error loading problems");
    (Status::Ok, Json(all_problems))
}

#[get("/<contest_id>/<num>")]
pub fn get_particular_problem(contest_id: String, num: String) -> (Status, Result<(), String>){
    let connection = &mut establish_connection();
    let p_num = num.parse::<i32>();
    if let Err(e) = p_num{
        return (Status::BadRequest, Err(format!("Error: {:?}", e)));
    }
    let problem_num = p_num.unwrap();
    // read the file at ./data/contest_id/problem_{problem_num}/problem.md
    let problem_file_path = Path::new("./data/").join(contest_id).join(format!("problem_{}", problem_num)).join("problem.md");
    let problem_data = std::fs::read_to_string(problem_file_path);

    if let Err(e) = problem_data{
        return (Status::InternalServerError, Err(String::from("No such contest or problem found"));
    }

    println!("Problem data: {:?}", problem_data.unwrap());

    (Status::Ok, Ok(()))
}