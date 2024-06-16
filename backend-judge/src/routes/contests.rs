use std::any::{type_name, type_name_of_val};
use std::ffi::OsStr;
use std::path::Path;
use rocket::{Data, FromForm, get, post, Response, State};
use crate::database::establish_connection;
use crate::models::{NewUser, User, LoginRequest, TokenResponse, ContestRequest};
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



use crate::check::checker;


#[derive(FromForm)]
struct ContestData<'f> {

    pub file: TempFile<'f>,
    pub data: Json<ContestRequest>
}
// upload a media zip file which will contain contest files
#[post("/file_upload", format = "multipart/form-data", data = "<formFields>")]
pub async fn create_contest(mut formFields: Form<ContestData<'_>>) -> (Status,Result<String, String> ){
    // Get raw file
    let mut form = formFields.into_inner();
    let file_name = form.file.raw_name().unwrap().dangerous_unsafe_unsanitized_raw().as_str();
    let data = form.data.into_inner();

    println!("data: {:?}", data);
    let new_id = Uuid::new_v4().to_string();

    if form.file.content_type().unwrap().extension().unwrap() != "zip" {
        return (Status::BadRequest, Err(String::from("File must be a zip file")));
    }
    // Generate new UUID
    let d: String = String::from("hello");
    // print type of id

    let save_file_name = new_id + ".zip";
    // Build path to save file
    let file_path = String::from("./media/")  + save_file_name.as_str();


    // Save file
    let som_file = form.file.persist_to(&file_path).await;
    if let Err(e) = som_file {
        return (Status::InternalServerError, Err(format!("Error: {:?}", e)));
    }

    let pass_status = checker(file_path.as_str(), data.num_problems, data.num_tests);

    if let Err(e) = pass_status{
        return (Status::BadRequest, Err(e));
    }


    (Status::Ok, Ok(String::from("Ok")))
}
