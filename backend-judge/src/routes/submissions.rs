use diesel::{BoolExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::{get, post};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::database::establish_connection;
use crate::models::{NewSubmission, NewSubmissionRequest, Problem, Submission};
use crate::submission_utils::{group_by_problem_id, group_by_user_id, run_tests};
use std::thread;
use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use crate::schema::problems::dsl::problems;
use crate::schema::submissions::dsl::submissions;
use crate::schema::test_cases::columns as test_case_columns;
use crate::schema::problems::columns as problem_columns;
use crate::schema::submissions::columns as submission_columns;
use crate::schema::users::columns as user_columns;
use crate::schema::test_results::columns as test_result_columns;
use diesel::ExpressionMethods;
use itertools::Itertools;
use crate::auth::AuthenticatedUser;
use crate::contest_utils::fetch_start_date;
use crate::responses::{GeneralSubmissionInfo, LeaderboardCell, LeaderboardProblem, LeaderboardRow, LeaderboardSubmissionInfo, SubmissionResponse, TestResultResponse};
use crate::schema::test_results::dsl::test_results;
use crate::schema::users::dsl::users;
use crate::schema::contests::{columns as contest_columns};
use crate::schema::contests::dsl::contests;
use crate::schema::test_cases::dsl::test_cases;


#[post("/", data = "<sub>")]
pub fn submit(authUser: AuthenticatedUser, sub: Json<NewSubmissionRequest>) -> (Status, Result<String, String>){
    let mut connection = establish_connection();
    let new_submission = NewSubmission::from_request(authUser, sub.into_inner());

    let contest_id = problems
        .select(problem_columns::contest_id)
        .filter(problem_columns::id.eq(new_submission.problem_id))
        .first::<String>(&mut connection);

    if let Err(e) = contest_id{
        return (Status::InternalServerError, Err(String::from("Error fetching contest id")));
    }

    let contest_id = contest_id.unwrap();
    let cur_time = fetch_start_date(contest_id.as_str());
    if let Err(_) = cur_time{
        return (Status::InternalServerError, Err(String::from("Error fetching contest start date")));
    }
    let check = crate::contest_utils::check_if_contest_available(cur_time.unwrap());
    if let Err(_) = check{
        return (Status::Forbidden, Err(String::from("Contest has not started yet, No Submissions allowed.")));
    }


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
    thread::spawn(move || run_tests(submission, problem));

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
        .inner_join(users.on(submission_columns::user_id.eq(user_columns::id)))
        .select((
            submission_columns::id,
            user_columns::username,
            submission_columns::extension,
            problem_columns::problem_num,
            problem_columns::name,
            submission_columns::created_at,
            submission_columns::verdict,
            submission_columns::time_taken
        ))
        .filter(submission_columns::id.eq(submission_id))
        .order_by(submission_columns::created_at.desc())
        .first::<GeneralSubmissionInfo>(connection);

    if let Err(e) = sub{
        return Err(format!("Error fetching submission: {}", e));
    }
    Ok(Json(sub.unwrap()))
}

#[get("/user/<user_name>")]
pub fn user_submissions(user_name: String) -> (Status, Result<Json<Vec<GeneralSubmissionInfo>>, String>){

    let mut connection = &mut establish_connection();
    let sub = submissions
        .inner_join(users.on(submission_columns::user_id.eq(user_columns::id)))
        .inner_join(problems.on(submission_columns::problem_id.eq(problem_columns::id)))
        .select((
            submission_columns::id,
            user_columns::username,
            submission_columns::extension,
            problem_columns::problem_num,
            problem_columns::name,
            submission_columns::created_at,
            submission_columns::verdict,
            submission_columns::time_taken
        ))
        .filter(user_columns::username.eq(user_name))
        .order_by(submission_columns::created_at.desc())
        .load::<GeneralSubmissionInfo>(connection);

    if let Err(e) = sub{
        return (Status::InternalServerError, Err(format!("Error fetching submission: {}", e)));
    }
    (Status::Ok, Ok(Json(sub.unwrap())))
}

#[get("/user_contest/<user_name>/<contest_id>")]
pub fn user_contest_submissions(user_name: String, contest_id: String) -> (Status, Result<Json<Vec<GeneralSubmissionInfo>>, String>){
    let connection = &mut establish_connection();
    println!("{}", user_name);
    let sub = submissions
        .inner_join(problems.on(submission_columns::problem_id.eq(problem_columns::id)))
        .inner_join(users.on(submission_columns::user_id.eq(user_columns::id)))
        .select((
            submission_columns::id,
            user_columns::username,
            submission_columns::extension,
            problem_columns::problem_num,
            problem_columns::name,
            submission_columns::created_at,
            submission_columns::verdict,
            submission_columns::time_taken
        ))
        .filter(user_columns::username.eq(user_name))
        .filter(problem_columns::contest_id.eq(contest_id))
        .order_by(submission_columns::created_at.desc())
        .load::<GeneralSubmissionInfo>(connection);

    if let Err(e) = sub{
        return (Status::InternalServerError, Err(format!("Error fetching submission: {}", e)));
    }
    (Status::Ok, Ok(Json(sub.unwrap())))
}

#[get("/contest/<contest_id>")]
pub fn contest_submissions(contest_id: String) -> (Status, Result<Json<Vec<GeneralSubmissionInfo>>, String>){
    let connection = &mut establish_connection();
    let sub = submissions
        .inner_join(problems.on(submission_columns::problem_id.eq(problem_columns::id)))
        .inner_join(users.on(user_columns::id.eq(submission_columns::user_id)))
        .select((
            submission_columns::id,
            user_columns::username,
            submission_columns::extension,
            problem_columns::problem_num,
            problem_columns::name,
            submission_columns::created_at,
            submission_columns::verdict,
            submission_columns::time_taken
        ))
        .filter(problem_columns::contest_id.eq(contest_id))
        .order_by(submission_columns::created_at.desc())
        .load::<GeneralSubmissionInfo>(connection);

    if let Err(e) = sub{
        return (Status::InternalServerError, Err(format!("Error fetching submission: {}", e)));
    }
    (Status::Ok, Ok(Json(sub.unwrap())))
}

#[get("/leaderboard/<contest_id>")]
pub fn leaderboard(contest_id: String) -> (Status, Result<Json<Vec<LeaderboardRow>>, String>){
    let connection = &mut establish_connection();
    let data = submissions
        .inner_join(problems.on(submission_columns::problem_id.eq(problem_columns::id)))
        .inner_join(users.on(user_columns::id.eq(submission_columns::user_id)))
        .select((
            user_columns::id,
            user_columns::username,
            submission_columns::id,
            problem_columns::problem_num,
            problem_columns::id,
            submission_columns::created_at,
            submission_columns::verdict,
            submission_columns::time_taken,
        ))
        .filter(problem_columns::contest_id.eq(contest_id))
        .load::<LeaderboardCell>(connection);

    if let Err(e) = data{
        return (Status::InternalServerError, Err(format!("Error fetching data: {}", e)));
    }
    fn problem_mapper((problem_id, cells) : (i32, Vec<LeaderboardCell>)) -> LeaderboardProblem{
        let problem_num = cells[0].problem_num;
        let var_submissions = cells.into_iter()
            .map(|cell| {
                LeaderboardSubmissionInfo::from_cell(cell)
            })
            .collect();
        LeaderboardProblem{
            problem_num,
            problem_id,
            submissions: var_submissions
        }
    }
    let cells = data.unwrap();
    // convert cells into LeaderboardRows

    let grouped_cells = group_by_user_id(cells);

    let rows:Vec<LeaderboardRow> = grouped_cells.into_iter()
        .map(|(user_id, cells)| {
            let username = cells[0].username.clone();
            let cells_by_problem = group_by_problem_id(cells);
            let cells : Vec<LeaderboardProblem>= cells_by_problem.into_iter()
                .map(problem_mapper)
                .collect();

            LeaderboardRow{
                username,
                cells
            }
        })
        .collect();

   (Status::Ok,  Ok(Json(rows)))

}

#[get("/particular/<submission_id>")]
pub fn particular_submission(submission_id: String) -> (Status,Result<Json<SubmissionResponse>, String>){
    let submission_id = match submission_id.parse::<i32>(){
        Ok(id) => id,
        Err(e) => return (Status::NotFound, Err(format!("Invalid submission id: {}", e)))
    };

    let connection = &mut establish_connection();
    let sub = submissions
        .select(Submission::as_select())
        .filter(submission_columns::id.eq(submission_id))
        .first::<Submission>(connection);

    let tests = test_results
        .inner_join(submissions.on(test_result_columns::submission_id.eq(submission_columns::id)))
        .inner_join(problems.on(submission_columns::problem_id.eq(problem_columns::id)))
        .inner_join(test_cases.on((problem_columns::contest_id.eq(test_case_columns::contest_id).and(problem_columns::problem_num.eq(test_case_columns::problem_num)).and(test_result_columns::test_num.eq(test_case_columns::test_num)))))
        .select((test_result_columns::test_num, test_result_columns::out, test_result_columns::verdict, test_result_columns::time_taken, test_case_columns::content))
        .filter(submission_columns::id.eq(submission_id))
        .load::<TestResultResponse>(connection);




    let end_date = problems
        .inner_join(submissions.on(problem_columns::id.eq(submission_columns::problem_id)))
        .inner_join(contests.on(problem_columns::contest_id.eq(contest_columns::id)))
        .select(contest_columns::end_date)
        .filter(submission_columns::id.eq(submission_id))
        .first::<NaiveDateTime>(connection);

    if let Err(e) = end_date{
        return (Status:: Forbidden, Err(format!("Error fetching end date: {}", e)));
    }

    let end_date = end_date.unwrap();


    if let Err(e) = sub{
        return (Status::InternalServerError, Err(format!("Error fetching submission: {}", e)));
    }

    if let Err(e) = tests{
        return (Status::InternalServerError, Err(format!("Error fetching test results: {}", e)));
    }

    let tests : Vec<TestResultResponse> = tests.unwrap();
    let sub = sub.unwrap();


    let response = SubmissionResponse::from_submission(&sub, tests);

    if response.created_at < end_date{
        return (Status::Forbidden, Err(String::from("Submission is not available yet")));
    }

    (Status::Ok, Ok(Json(response)))
}