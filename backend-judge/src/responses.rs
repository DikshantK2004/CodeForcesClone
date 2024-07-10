use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use rocket::Responder;
use rocket::serde::Serialize;
use crate::models::SampleTestCase;
use crate::schema::*;
#[derive(Debug, Serialize, Responder)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ProblemResponse{
    pub info: GeneralProblemInfo,
    pub statement: String,
    pub samples: Option<Vec<SampleTestCase>>
}


#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Clone)]
#[diesel(table_name = problems)]
pub struct GeneralProblemInfo{
    pub id: i32,
    pub name: String,
    pub problem_num: i32,
    pub contest_id: String,
    pub num_samples: i32,
    pub time_limit: i32,
    pub accepted:i32,
}

#[derive(Debug, Queryable, Serialize, Clone)]
pub struct GeneralSubmissionInfo{
    pub id: i32,
    pub username: String,
    pub problem_id: i32,
    pub problem_name: String,
    pub created_at: NaiveDateTime,
    pub verdict: String,
    pub time_taken: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct LeaderboardSubmissionInfo{
    pub submission_id: i32,
    pub created_at: NaiveDateTime,
    pub verdict: String,
    pub time_taken: Option<i32>
}

#[derive(Debug, Serialize)]
pub struct LeaderboardRow{
    pub username: String,
    pub cells: Vec<LeaderboardProblem>
}

#[derive(Debug, Serialize)]
pub struct LeaderboardProblem{
    pub problem_num: i32,
    pub problem_id: i32,
    pub submissions: Vec<LeaderboardSubmissionInfo>
}
#[derive(Debug, Serialize, Queryable)]
pub struct LeaderboardCell{
    pub user_id: i32,
    pub username: String,
    pub submission_id: i32,
    pub problem_num: i32,
    pub problem_id: i32,
    pub created_at: NaiveDateTime,
    pub verdict: String,
    pub time_taken: Option<i32>
}

impl LeaderboardSubmissionInfo{
    pub fn from_cell(cell: LeaderboardCell) -> Self{
        LeaderboardSubmissionInfo{
            submission_id: cell.submission_id,
            created_at: cell.created_at,
            verdict: cell.verdict,
            time_taken: cell.time_taken
        }
    }
}



#[derive(Debug, Serialize, Selectable, Queryable,Clone)]
#[diesel(table_name = test_results)]
pub struct TestResultResponse{
    pub test_num: i32,
    pub out: String,
    pub verdict: String,
    pub time_taken: i32
}

#[derive(Debug, Serialize)]
pub struct SubmissionResponse{
    pub id: i32,
    pub code: String,
    pub extension: String,
    pub problem_id: i32,
    pub created_at: NaiveDateTime,
    pub verdict: String,
    pub time_taken: Option<i32>,
    pub test_results: Vec<TestResultResponse>
}

impl SubmissionResponse{
    pub fn from_submission(submission: &crate::models::Submission, test_results: Vec<TestResultResponse>) -> Self{
        SubmissionResponse{
            id: submission.id,
            code: submission.code.clone(),
            extension: submission.extension.clone(),
            problem_id: submission.problem_id,
            created_at: submission.created_at,
            verdict: submission.verdict.clone(),
            time_taken: submission.time_taken,
            test_results
        }
    }
}