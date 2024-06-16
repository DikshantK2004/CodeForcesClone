use std::any::{type_name, type_name_of_val};
use std::ffi::OsStr;
use std::path::Path;
use rocket::{Data, FromForm, get, post, Response, State};
use crate::database::establish_connection;
use crate::models::{NewUser, User, LoginRequest, TokenResponse};
// import Status
use rocket::http::Status;
use rocket::serde::json::Json;
use diesel::result;
use diesel::result::{DatabaseErrorKind, Error};
use crate::responses::MessageResponse;
use rocket::response::status;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::sql_types::Uuid;
use rocket::data::ToByteUnit;
use rocket::form::Form;
use rocket::fs::TempFile;
use crate::utils::*;

//
// #[get("/<id>")]
// pub fn get_contest(id: i32) -> (Status, Json<MessageResponse>){
//     (Status::Ok, Json(MessageResponse {
//         message: format!("Getting contest with id: {:?}", id)
//     }))
// }

#[derive(FromForm)]
struct Upload<'f> {
    file: TempFile<'f>
}
// upload a media zip file which will contain contest files
#[post("/file_upload", format = "multipart/form-data", data = "<form>")]
pub async fn create_contest(mut form: Form<Upload<'_>>) -> (Status,Result<String, String> ){
    // Get raw file
    let file_name = form.file.raw_name().unwrap().dangerous_unsafe_unsanitized_raw().as_str();
    // Get extension of file name
    let extension = Path::new(file_name).extension().and_then(OsStr::to_str).unwrap();
    // Generate new UUID
    let d: String = String::from("hello");
    // print type of id


    // Build path to save file
    let file_path = String::from("./media/")  + &d + "." + extension;
    println!("name of file: {:?}", file_path);
    // Save file
    let som_file = form.file.persist_to(file_path).await;
    if let Err(e) = som_file {
        return (Status::InternalServerError, Err(format!("Error: {:?}", e)));
    }

    (Status::Ok, Ok(String::from("Ok")))
}
