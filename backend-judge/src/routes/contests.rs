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
use crate::responses::{GeneralProblemInfo, MessageResponse};
use rocket::response::status;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::query_dsl::InternalJoinDsl;
use rocket::data::ToByteUnit;
use rocket::form::Form;
use rocket::fs::TempFile;
use crate::utils::*;
use uuid::Uuid;
use crate::auth::AuthenticatedUser;
use crate::models::GeneralContestInfo;

//
// #[get("/<id>")]
// pub fn get_contest(id: i32) -> (Status, Json<MessageResponse>){
//     (Status::Ok, Json(MessageResponse {
//         message: format!("Getting contest with id: {:?}", id)
//     }))
// }

use crate::contest_utils::*;
use crate::schema::contests::dsl::contests;

// upload a media zip file which will contain contest files
#[post("/create", format = "multipart/form-data", data = "<formFields>")]
pub async fn create_contest(authUser: AuthenticatedUser, formFields: Form<ContestData<'_>>) -> (Status,Result<String, String> ){
    // Get raw file
    let mut form = formFields.into_inner();
    let user_id = authUser.0;
    let file_name = form.file.raw_name().unwrap().dangerous_unsafe_unsanitized_raw().as_str();
    let data = form.data.into_inner();
    let new_id = Uuid::new_v4().to_string();
    if data.problem_names.len() != data.num_problems as usize{
        return (Status::BadRequest, Err(String::from("Number of problems and problem names do not match")));
    }


    if form.file.content_type().unwrap().extension().unwrap() != "zip" {
        return (Status::BadRequest, Err(String::from("File must be a zip file")));
    }
    // Generate new UUID
    let save_file_name = new_id.as_str();
    let file_path = String::from("./media/")  + save_file_name + ".zip";

    // Save file
    let som_file = form.file.persist_to(&file_path).await;
    if let Err(e) = som_file {
        return (Status::InternalServerError, Err(format!("Error: {:?}", e)));
    }

    let pass_status = checker(file_path.as_str(), &data.num_problems, &data.num_tests, &data.num_samples, &data.time_limits);

    if let Err(e) = pass_status{
        return (Status::BadRequest, Err(e));
    }


    // unzip the archive in ./data/save_file_name using cli commands
    let unzip_status = extract_zip(file_path.as_str(), save_file_name);

    if let Err(e) = unzip_status{
        return (Status::InternalServerError, Err(e));
    }


    match compile_validators(new_id.as_str(), data.num_problems){
        Err(e) => return (Status::InternalServerError, Err(e)),
        _ => {}
    }

    // save the contest in the database
    let connection = &mut establish_connection();

    let num_tests = data.num_tests();
    let prob_names = data.prob_names();
    let num_samples = data.num_samples();
    let time_limits = data.time_limits();
    // gives up ownership of data
    let contest = Contest::from_request(new_id.as_str(), data, user_id);

    diesel::insert_into(crate::schema::contests::table)
        .values(&contest)
        .execute(connection)
        .expect("Error saving contest");

    // save the problems in the db
    insert_problems_to_db(contest.num_problems,time_limits ,prob_names, num_tests, num_samples, new_id.clone(), connection);



    let remove_zip_status = remove_zip(file_path.as_str());
    if let Err(e) = remove_zip_status{
        return (Status::InternalServerError, Err(e));
    }

    (Status::Ok, Ok(format!("Contest created successfully: {} ", new_id)))
}


#[put("/update/<contest_id>", format = "multipart/form-data", data = "<formFields>")]
pub async fn update_contest(authUser: AuthenticatedUser,contest_id: String, formFields: Form<ContestData<'_>>) -> (Status,Result<String, String> ) {
    // Get raw file
    let user_id = authUser.0;
    let mut form = formFields.into_inner();
    let data = form.data.into_inner();


    if form.file.content_type().unwrap().extension().unwrap() != "zip" {
        return (Status::BadRequest, Err(String::from("File must be a zip file")));
    }

    if data.problem_names.len() != data.num_problems as usize{
        return (Status::BadRequest, Err(String::from("Number of problems and problem names do not match")));
    }
    // Generate new UUID
    let save_file_name = contest_id.as_str();
    let file_path = String::from("./media/") + save_file_name + ".zip";

    // Save file
    let som_file = form.file.persist_to(&file_path).await;
    if let Err(e) = som_file {
        return (Status::InternalServerError, Err(format!("Error: {:?}", e)));
    }

    // checking if new zip follows the correct format
    let pass_status = checker(file_path.as_str(), &data.num_problems, &data.num_tests, &data.num_samples, &data.time_limits);
    if let Err(e) = pass_status{
        return (Status::BadRequest, Err(e));
    }

    // remove the existing files
    let remove_status = remove_existing_contest(contest_id.as_str());
    if let Err(e) = remove_status{
        return (Status::InternalServerError, Err(e));
    }

    // unzip the archive in ./data/save_file_name using cli commands
    let unzip_status = extract_zip(file_path.as_str(), save_file_name);
    if let Err(e) = unzip_status{
        println!("Error unzipping file: {:?}", e);
        return (Status::InternalServerError, Err(e));
    }

    match compile_validators(contest_id.as_str(), data.num_problems){
        Err(e) => return (Status::InternalServerError, Err(e)),
        _ => {}
    }



    // update the contest data in db
    let connection = & mut establish_connection();
    let num_tests = data.num_tests();
    let prob_names = data.prob_names();
    let num_samples = data.num_samples();
    let time_limits = data.time_limits();

    let contest = Contest::from_request(contest_id.as_str(), data, user_id);
    let update_status = diesel::update(crate::schema::contests::table.find(contest_id.clone()))
        .set(&contest)
        .execute(connection);

    println!("hello {:?}", update_status);

    if let Err(e) = update_status{
        return (Status::InternalServerError, Err(format!("Error updating contest: {:?}", e)));
    }

    // delete existing problems
    let delete_status = diesel::delete(crate::schema::problems::table.filter(crate::schema::problems::contest_id.eq(contest_id.clone())))
        .execute(connection);

    if let Err(e) = delete_status{
        return (Status::InternalServerError, Err(format!("Error deleting problems: {:?}", e)));
    }

    // save the problems in the db
    insert_problems_to_db(contest.num_problems, time_limits,prob_names, num_tests, num_samples, contest_id.clone(), connection);



    let remove_zip_status = remove_zip(file_path.as_str());

    if let Err(e) = remove_zip_status{
        return (Status::InternalServerError, Err(e));
    }

    (Status::Ok, Ok(String::from("Contest updated successfully ")))

}

#[get("/all")]
pub fn get_all_contests() -> Result<Json<Vec<GeneralContestInfo>>, String>{
    let connection = &mut establish_connection();
    let results =  crate::schema::contests::table.select(GeneralContestInfo::as_select())
        .order_by(crate::schema::contests::start_date.desc())
        .load:: <GeneralContestInfo>(connection);

    if let Err(e) = results{
        return Err(format!("Error getting contests: {:?}", e));
    }

    let data = results.unwrap();


    Ok(Json(data))

}

#[get("/particular/<contest_id>")]
pub fn get_particular_contest(contest_id: String) -> (Status,Result<Json<ContestResponse>, String>){
    let connection = &mut establish_connection();
    let results =  crate::schema::contests::table.inner_join(crate::schema::problems::table)
                    .filter(crate::schema::contests::id.eq(contest_id.clone()))
                    .select((crate::schema::contests::all_columns, GeneralProblemInfo::as_select()))
        .order_by(crate::schema::problems::problem_num.asc())
                    .load::<(Contest, GeneralProblemInfo)>(connection);

    if let Err(e) = results{
        return (Status::InternalServerError, Err(format!("Error getting contest: {:?}", e)));
    }



    let data = results.unwrap();
    if data.len() == 0{
        return (Status::NotFound, Err(String::from("No such contest found")));
    }

    if let Err(_) = check_if_contest_available(data[0].0.start_date){
        return (Status::Forbidden,Err("Contest has not started yet".parse().unwrap()));
    }


    let res = ContestResponse::from_contest(data[0].0.clone(), data.iter().map(|x| GeneralProblemInfo::from(x.1.clone())).collect());

    (Status::Ok,Ok(Json(res)))
}