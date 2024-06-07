use diesel::RunQueryDsl;
use rocket::{get, post};
use crate::database::establish_connection;
use crate::models:: {User, NewUser};
// import Status
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/")]
pub fn index() -> &'static str {

    println!("Hello, world!");
    "Hello, world!"
}

#[post("/create-fresh", data = "<user>")]
pub fn create(user: Json<NewUser>) -> Result<Json<User>, Status> {
    let new_user =  user.into_inner();

    println!("{:#?}", new_user);
    let connection = &mut  establish_connection();
    let user = diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        .get_result(connection)
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(user))

}