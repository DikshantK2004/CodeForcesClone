use diesel::{JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::{get, post};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::database::establish_connection;
use crate::models::{NewSubmission, Problem, Submission};
use crate::submission_utils::run_tests;
use std::thread;
use diesel::associations::HasTable;
use crate::schema::problems::dsl::problems;
use crate::schema::submissions::dsl::submissions;
use crate::schema::problems::columns as problem_columns;
use crate::schema::submissions::columns as submission_columns;
use diesel::ExpressionMethods;
use crate::responses::GeneralSubmissionInfo;

#[post("/", data = "<sub>")]
pub fn submit(sub: Json<NewSubmission>) -> (Status, Result<String, String>){
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
    let id = submission.id;
    // start the function validate on a new thread and continue beyond this point to response
    thread::spawn(move || run_tests(submission, problem.contest_id, problem.problem_num, problem.num_tests));

    (Status:: Ok,Ok(format!("{}",id)))
}

#[get("/general/<id>")]
pub fn general_submission_handler(id: String) -> Result<Json<GeneralSubmissionInfo>, String>{
    let submission_id = match id.parse::<i32>(){
        Ok(id) => id,
        Err(e) => return Err(format!("Invalid submission id: {}", e))
    };


    let connection = &mut establish_connection();
    let sub = submissions
        .inner_join(problems.on(submission_columns::problem_id.eq(problem_columns::id)))
        .select((
            submission_columns::id,
            submission_columns::problem_id,
            problem_columns::name,
            submission_columns::created_at,
            submission_columns::verdict
        ))
        .filter(submission_columns::id.eq(submission_id))
        .first::<GeneralSubmissionInfo>(connection);

    if let Err(e) = sub{
        return Err(format!("Error fetching submission: {}", e));
    }
    Ok(Json(sub.unwrap()))
}

#[get("/user/<id>")]
pub fn user_submissions(id: String) -> Result<Json<Vec<GeneralSubmissionInfo>>, String>{
    let user_id = match id.parse::<i32>(){
        Ok(id) => id,
        Err(e) => return Err(format!("Invalid user id: {}", e))
    };

    let mut connection = &mut establish_connection();
    let sub = submissions
        .inner_join(problems.on(submission_columns::problem_id.eq(problem_columns::id)))
        .select((
            submission_columns::id,
            submission_columns::problem_id,
            problem_columns::name,
            submission_columns::created_at,
            submission_columns::verdict
        ))
        .filter(submission_columns::user_id.eq(user_id))
        .load::<GeneralSubmissionInfo>(connection);

    if let Err(e) = sub{
        return Err(format!("Error fetching submission: {}", e));
    }
    Ok(Json(sub.unwrap()))
}