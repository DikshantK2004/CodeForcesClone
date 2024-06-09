yResult, RunQueryDsl, SelectableHelper};
use rocket::{get, post, Response};
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
use crate::utils::*;


#[get("/<id>")]
pub fn get_contest(id: i32) -> (Status, Json<MessageResponse>){
    (Status::Ok, Json(MessageResponse {
        message: format!("Getting contest with id: {}", id)
    }))
}

#[post("/create", data = "<contest>")]
pub fn create_contest()