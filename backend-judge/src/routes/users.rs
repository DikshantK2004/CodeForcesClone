use diesel::{QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
use rocket::{get, post, Response};
use crate::database::establish_connection;
use crate::models::{NewUser, User, LoginRequest, TokenResponse};
// import Status
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;
use diesel::result;
use diesel::result::{DatabaseErrorKind, Error};
use crate::responses::MessageResponse;
use rocket::response::status;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use crate::utils::*;
use crate::auth::*;

#[get("/")]
pub fn index() -> &'static str {

    println!("Hello, world!");
    "Hello, world!"
}

#[post("/register", data = "<user>")]
pub fn create(user: Json<NewUser>) -> (Status, Result<String, String>){
    let mut new_user : NewUser =  user.into_inner();
    new_user.password = hash_password(&new_user.password);
    let connection = &mut  establish_connection();
    let user:QueryResult<User> = diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        .get_result(connection);
    match user {
        Ok(user) => {
            println!("User created: {:?}", user);

            (Status::Created, Ok("User created".to_string()))
        },
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) =>{
            // return a message with status

            (Status::Conflict, Err("User already exists".to_string())
            )
        }

        Err(_) => {
            println!("Failed to create user");
            (Status::InternalServerError, Err("Internal Server Error".to_string()))
        }
    }


}

#[post("/login", data = "<payload>")]
pub fn login(payload: Json<LoginRequest>, cookies: &CookieJar<'_>) -> (Status, Result<String, String>){
    let connection = &mut establish_connection();
    let data = payload.into_inner();

    // Attempt to retrieve the user from the database
    let user_result = users
        .filter(email.eq(&data.email))
        .select(User::as_select())
        .get_result::<User>(connection);

    let user = match user_result {
        Ok(user) => user,
        Err(diesel::result::Error::NotFound) => {
            println!("User not found");
            return (Status::NotFound, Err( "User not found".to_string()));
        },
        Err(_) => {
            println!("Database error occurred");
            return (Status::InternalServerError, Err("Internal Server Error".to_string()));
        }
    };

    // Check if the password is correct
    if !verify_password(&data.password, &user.password) {
        println!("Invalid password");
        return (Status::Forbidden, Err("Invalid password".to_string()));
    }

    // Attempt to create a JWT for the user
    let jwt = match create_jwt(user.id) {
        Ok(jwt) => jwt,
        Err(_) => {
            println!("Failed to create JWT");
            return (Status::InternalServerError, Err( "Internal Server Error".to_string()));
        }
    };

    println!("Login successful: User ID {}", user.id);
    cookies.add(Cookie::new("token", jwt.clone()));
    cookies.add(Cookie::new("username", user.username.clone()));
    (Status::Ok, Ok(user.username))
}


#[post("/guard")]
pub fn guard(auth_usr: AuthenticatedUser)-> Status{
Status::Ok
}