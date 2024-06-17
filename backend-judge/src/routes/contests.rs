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
use rocket::form::Form;
use rocket::fs::TempFile;
use crate::utils::*;
use uuid::Uuid;

//
// #[get("/<id>")]
// pub fn get_contest(id: i32) -> (Status, Json<MessageResponse>){
//     (Status::Ok, Json(MessageResponse {
//         message: format!("Getting contest with id: {:?}", id)
//     }))
// }

use crate::contest_utils::*;

// upload a media zip file which will contain contest files
#[post("/create", format = "multipart/form-data", data = "<formFields>")]
pub async fn create_contest(formFields: Form<ContestData<'_>>) -> (Status,Result<String, String> ){
    // Get raw file
    let mut form = formFields.into_inner();
    let file_name = form.file.raw_name().unwrap().dangerous_unsafe_unsanitized_raw().as_str();
    let data = form.data.into_inner();
    let new_id = Uuid::new_v4().to_string();

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

    let pass_status = checker(file_path.as_str(), &data.num_problems, &data.num_tests);

    if let Err(e) = pass_status{
        return (Status::BadRequest, Err(e));
    }


    // save the contest in the database
    let connection = &mut establish_connection();

    let num_tests = data.num_tests();
    // gives up ownership of data
    let contest = Contest::from_request(new_id.as_str(), data);

    diesel::insert_into(crate::schema::contests::table)
        .values(&contest)
        .execute(connection)
        .expect("Error saving contest");

    // save the problems in the db

    for i in 0..contest.num_problems{
        let new_problem_id = Uuid::new_v4().to_string();
        let problem = Problem{
            id: new_problem_id,
            num_tests: num_tests[i as usize],
            contest_id: new_id.clone()
        };

        diesel::insert_into(crate::schema::problems::table)
            .values(&problem)
            .execute(connection)
            .expect("Error saving problem");
    }
    // unzip the archive in ./data/save_file_name using cli commands
    let unzip_status = extract_zip(file_path.as_str(), save_file_name);

    if let Err(e) = unzip_status{
        return (Status::InternalServerError, Err(e));
    }

    let remove_zip_status = remove_zip(file_path.as_str());
    if let Err(e) = remove_zip_status{
        return (Status::InternalServerError, Err(e));
    }

    (Status::Ok, Ok(String::from("Contest created successfully ")))
}


#[put("/update/<contest_id>", format = "multipart/form-data", data = "<formFields>")]
pub async fn update_contest(contest_id: String, formFields: Form<ContestData<'_>>) -> (Status,Result<String, String> ) {
    // Get raw file
    let mut form = formFields.into_inner();
    let file_name = form.file.raw_name().unwrap().dangerous_unsafe_unsanitized_raw().as_str();
    let data = form.data.into_inner();


    if form.file.content_type().unwrap().extension().unwrap() != "zip" {
        return (Status::BadRequest, Err(String::from("File must be a zip file")));
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
    let pass_status = checker(file_path.as_str(), &data.num_problems, &data.num_tests);
    if let Err(e) = pass_status{
        return (Status::BadRequest, Err(e));
    }

    // update the contest data in db
    let connection = &mut establish_connection();
    let num_tests = data.num_tests();

    let contest = Contest::from_request(contest_id.as_str(), data);
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
    for i in 0..contest.num_problems{
        let new_problem_id = Uuid::new_v4().to_string();
        let problem = Problem{
            id: new_problem_id,
            num_tests: num_tests[i as usize],
            contest_id: contest_id.clone()
        };

        diesel::insert_into(crate::schema::problems::table)
            .values(&problem)
            .execute(connection)
            .expect("Error saving problem");
    }


    // remove the existing files
    let remove_status = remove_existing_contest(contest_id.as_str());
    if let Err(e) = remove_status{
        return (Status::InternalServerError, Err(e));
    }

    // unzip the archive in ./data/save_file_name using cli commands
    let unzip_status = extract_zip(file_path.as_str(), save_file_name);
    if let Err(e) = unzip_status{
        return (Status::InternalServerError, Err(e));
    }

    let remove_zip_status = remove_zip(file_path.as_str());

    if let Err(e) = remove_zip_status{
        return (Status::InternalServerError, Err(e));
    }

    (Status::Ok, Ok(String::from("Contest updated successfully ")))

}

#[get("/contests/")]
