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
}

#[derive(Debug, Queryable, Serialize, Clone)]
pub struct GeneralSubmissionInfo{
    pub id: i32,
    pub problem_id: i32,
    pub problem_name: String,
    pub created_at: NaiveDateTime,
    pub verdict: String,
    pub time_taken: Option<i32>,
}
