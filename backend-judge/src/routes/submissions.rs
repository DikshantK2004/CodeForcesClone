use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::{get, post};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::database::establish_connection;
use crate::models::{NewSubmission, Problem, Submission};
use crate::submission_utils::run_tests;


#[post("/", data = "<sub>")]
pub fn submit(sub: Json<NewSubmission>) -> (Status, Result<(), String>){
    let mut connection = establish_connection();
    let new_submission = sub.into_inner();


    // fetch the problem
    let problem = crate::schema::problems::table.find(new_submission.problem_id)
        .select(Problem::as_select())
        .first::<crate::models::Problem>(&mut connection);
    if let Err(e) = problem{
        return (Status::InternalServerError, Err(String::from("Error fetching problem")));
    }
    let problem = problem.unwrap();
    println!("Problem fetched: {:?}", problem);


    let sub_res = diesel::insert_into(crate::schema::submissions::table)
        .values(&new_submission)
        .get_result::<Submission>(&mut connection);
    if let Err(e) = sub_res{
        println!("Error saving submission: {:?}", e);
        return (Status::InternalServerError, Err(String::from("Submission id could not be generated")));
    }
    let submission = sub_res.unwrap();

    // start the function validate on a new thread and continue beyond this point to response
    run_tests(&submission, &problem.contest_id, problem.problem_num, problem.num_tests);



    (Status:: Ok,Ok(()))
}