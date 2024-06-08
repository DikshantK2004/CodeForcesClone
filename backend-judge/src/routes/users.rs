use diesel::{QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
use rocket::{get, post, Response};
use crate::database::establish_connection;
use crate::models:: {NewUser, User, LoginRequest};
// import Status
use rocket::http::Status;
use rocket::serde::json::Json;
use diesel::result;
use diesel::result::{DatabaseErrorKind, Error};
use crate::responses::MessageResponse;
use rocket::response::status;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use crate::utils::{status_message, verify_password};

#[get("/")]
pub fn index() -> &'static str {

    println!("Hello, world!");
    "Hello, world!"
}

#[post("/register", data = "<user>")]
pub fn create(user: Json<NewUser>) -> (Status, Json<MessageResponse>){
    let mut new_user : NewUser =  user.into_inner();
    new_user.password = crate::utils::hash_password(&new_user.password);
    let connection = &mut  establish_connection();
    let user:QueryResult<User> = diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        .get_result(connection);
    match user {
        Ok(user) => {
            println!("User created: {:?}", user);

            (Status::Created, Json(MessageResponse {
                message: "User created".to_string()
            }))
        },
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) =>{
            println!("User already exists");
            // return a message with status

            (Status::Conflict, Json(MessageResponse {
                message: "User with email already already exists".to_string()
            })
            )
        }

        Err(_) => {
            println!("Failed to create user");
            (Status::InternalServerError, Json(MessageResponse {
                message: "Internal Server Error: Failed to create user".to_string()
            }))
        }
    }


}

#[post("/login", data = "<payload>")]
pub fn login(payload: Json<LoginRequest>) -> (Status, Json<MessageResponse>){
    let connection = &mut establish_connection();
    let data = payload.into_inner();
    let res= users
        .filter(email.eq(&data.email))
        .select(User::as_select())
        .get_result(connection);

    if let Err(err) = res {
        if let Error::NotFound = err {
            println!("User not found");
            return status_message(Status::NotFound, "User not found");
        } else {
            println!("Internal Server Error: {:?}", err);
            return status_message(Status::InternalServerError, "Internal Server Error");
        }
    }

    let user = res.unwrap();
    if verify_password(&data.password, &user.password) {
        println!("User found: {:?}", user);
        status_message(Status::Ok, "User found")
    } else {
        println!("Invalid password");
        status_message(Status::Unauthorized, "Invalid password")
    }

}