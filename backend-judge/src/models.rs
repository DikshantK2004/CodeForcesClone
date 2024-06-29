use std::str::FromStr;
use chrono::{NaiveDateTime};
use diesel::prelude::*;
use rocket::data::FromData;
use rocket::serde::{Deserialize, Serialize};
use crate::schema::{users, problems, contests, submissions, test_results};
use rocket::{FromForm};
use rocket::fs::TempFile;
use rocket::serde::json::Json;
use crate::responses::GeneralProblemInfo;

// This struct represents a row in the `users` table
#[derive(Queryable,Debug, Serialize, Deserialize, Selectable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub verified: bool,
}

// This struct is used for inserting a new row into the `users` table
#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub verified: Option<bool>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContestRequest{
    pub name: String,
    pub description: String,
    pub start_date: String,
    pub end_date: String,
    pub creator_id: i32,
    pub num_problems: i32,
    pub problem_names: Vec<String>,
    pub num_tests: Vec<i32>,
    pub num_samples: Vec<i32>
    // add a check that its from particular langauages only

    // pub solution_ext: String,
}
impl ContestRequest{
    pub fn num_tests(&self) -> Vec<i32>{
        self.num_tests.clone()
    }

    pub fn num_samples(&self) -> Vec<i32>{
        self.num_samples.clone()
    }

    pub fn prob_names(&self) -> Vec<String>{
        self.problem_names.clone()
    }
}

#[derive(Debug, Serialize, Queryable, Selectable, Insertable, AsChangeset, Clone)]
pub struct Contest{
    pub id: String,
    pub name: String,
    pub description: String,
    pub num_problems: i32,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub creator_id: i32,
}


impl Contest{
    pub fn from_request(id: &str, req: ContestRequest) -> Contest{
        Contest{
            id: id.to_string(),
            name: req.name,
            description: req.description,
            start_date: NaiveDateTime::parse_from_str(&req.start_date, "%d-%m-%Y %H:%M:%S").unwrap(),
            end_date: NaiveDateTime::parse_from_str(&req.end_date, "%d-%m-%Y %H:%M:%S").unwrap(),
            creator_id: req.creator_id,
            num_problems: req.num_problems,
            created_at: None,
            updated_at: None
        }
    }
}
#[derive(Debug, Queryable, Selectable, Insertable, Serialize)]
pub struct Problem{
    pub name: String,
    pub problem_num: i32,
    pub num_tests: i32,
    pub num_samples: i32,
    pub contest_id: String,
}



#[derive(Debug, Serialize, Selectable, Queryable)]
#[diesel(table_name = contests)]
pub struct GeneralContestInfo{
    pub id: String,
    pub name: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub num_problems: i32,
}


#[derive(FromForm)]
pub struct ContestData<'f> {
    pub file: TempFile<'f>,
    pub data: Json<ContestRequest>
}


#[derive(Serialize)]
pub struct ContestResponse{
    pub id: String,
    pub name: String,
    pub description: String,
    pub num_problems: i32,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub problems: Vec<GeneralProblemInfo>
}

impl ContestResponse{
    pub fn from_contest(contest: Contest, problems: Vec<GeneralProblemInfo>) -> ContestResponse{
        ContestResponse{
            id: contest.id,
            name: contest.name,
            description: contest.description,
            num_problems: contest.num_problems,
            start_date: contest.start_date,
            end_date: contest.end_date,
            created_at: contest.created_at,
            updated_at: contest.updated_at,
            problems
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SampleTestCase{
    pub input: String,
    pub output: String
}


#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = submissions)]
pub struct NewSubmission{
    pub code: String,
    pub extension: String,
    pub user_id: i32,
    pub problem_id: i32,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Insertable)]
pub struct Submission{
    pub id: i32,
    pub code: String,
    pub extension: String,
    pub user_id: i32,
    pub problem_id: i32,
    pub created_at: NaiveDateTime,
    pub verdict: String,
    pub time_taken: Option<i32>
}


#[derive(Debug, Serialize, Insertable)]
#[diesel(table_name = test_results)]
pub struct NewTestResult{
    pub submission_id: i32,
    pub test_num: i32,
    pub out: String,
    pub verdict: String,
    pub time_taken: i32,
}
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ProblemInput{
//     pub name: String,
//     pub content: String,
//     pub test_cases: Vec<TestCase>
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct TestCase{
//     pub input: String,
//     pub output: String
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Contest{
//     pub id: i32,
//     pub name: String,
//     pub description: String,
//     pub start_date: chrono::Utc::datetime,
//     pub end_date: chrono::Utc::datetime,
//     pub creator_id: i32,
//     pub problems: Option<Vec<Problem>>
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Problem{
//     pub id: i32,
//     pub name: String,
//     pub content: String,
//     pub test_cases: Option<Vec<TestCase>>
// }