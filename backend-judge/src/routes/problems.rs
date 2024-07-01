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
use crate::responses::{GeneralProblemInfo, MessageResponse, ProblemResponse};
use rocket::response::status;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use rocket::data::ToByteUnit;
use rocket::figment::Source::File;
use rocket::form::Form;
use rocket::fs::TempFile;
use crate::utils::*;
use uuid::Uuid;

use crate::contest_utils::*;
use crate::schema::contests::dsl::contests;
use crate::schema::problems::dsl::problems;

use crate::schema::problems::columns as problem_columns;
use crate::schema::contests::columns as contest_columns;

#[get("/all")]
pub fn get_all_problems() -> (Status, Result<Json<Vec<GeneralProblemInfo>>, String>){

    let connection = &mut establish_connection();
    let cur_time = crate::utils::get_current_ist();
    let all_problems = problems
        .inner_join(contests.on(problem_columns::contest_id.eq(contest_columns::id)))
        .select(GeneralProblemInfo::as_select())
        .filter(contest_columns::start_date.le(cur_time))
        .order_by(contest_columns::start_date.desc())
        .load(connection);

    if let Err(e) = all_problems{
        return (Status::InternalServerError, Err(format!("Error: {:?}", e)));
    }
    let probs = all_problems.unwrap();

    (Status::Ok, Ok(Json(probs)))
}

#[get("/<contest_id>/<num>")]
pub fn get_particular_problem(contest_id: String, num: String) -> (Status, Result<Json<ProblemResponse>, String>){

    let start_date = fetch_start_date(contest_id.as_str());

    if let Err(_) = start_date{
        return (Status::InternalServerError, Err(String::from("Error fetching contest start date")));
    }
    let check = check_if_contest_available(start_date.unwrap());
    if let Err(_) = check{
        return (Status::Forbidden, Err(String::from("Contest has not started yet")));
    }

    let connection = &mut establish_connection();
    let p_num = num.parse::<i32>();
    if let Err(e) = p_num{
        return (Status::BadRequest, Err(format!("Error: {:?}", e)));
    }
    let problem_num = p_num.unwrap();
    // read the file at ./data/contest_id/problem_{problem_num}/problem.md
    let problem_file_path = Path::new("./data/").join(contest_id.as_str()).join(format!("problem_{}", problem_num)).join("problem.md");
    let problem_data = std::fs::read_to_string(problem_file_path);

    if let Err(e) = problem_data{
        return (Status::InternalServerError, Err(String::from("No such contest or problem found")));
    };

    let problem_info = problems
        .filter(crate::schema::problems::contest_id.eq(contest_id.as_str()) )
        .filter(crate::schema::problems::problem_num.eq(problem_num))
        .select(GeneralProblemInfo::as_select())
        .first(connection);

    if let Err(e) = problem_info{
        return (Status::InternalServerError, Err(String::from("No such contest or problem found")));
    }

    let samples = gather_samples(contest_id.as_str(), problem_num, problem_info.as_ref().unwrap().num_samples);

    if let Err(e) = samples{
        return (Status::InternalServerError, Ok(Json(ProblemResponse{
            info: problem_info.unwrap(),
            statement: problem_data.unwrap(),
            samples: None,
        })));
    };

    (Status::Ok, Ok(Json(ProblemResponse{
        info: problem_info.unwrap(),
        statement: problem_data.unwrap(),
        samples: Some(samples.unwrap()),
    })))


}