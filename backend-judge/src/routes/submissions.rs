use diesel::RunQueryDsl;
use rocket::{get, post};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::database::establish_connection;
use crate::models::{NewSubmission, Submission};


#[post("/", data = "<sub>")]
pub fn submit(sub: Json<NewSubmission>) -> (Status, Result<(), String>){
    let mut connection = establish_connection();
    let new_submission = sub.into_inner();
    let sub_res = diesel::insert_into(crate::schema::submissions::table)
        .values(&new_submission)
        .get_result::<Submission>(&mut connection);
    if let Err(e) = sub_res{
        println!("Error saving submission: {:?}", e);
        return (Status::InternalServerError, Err(String::from("Submission id could not be generated")));
    }
    let submission = sub_res.unwrap();




    (Status:: Ok,Ok(()))
}