use rocket::{get, post};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::database::establish_connection;
use crate::models::{NewSubmission, Submission};