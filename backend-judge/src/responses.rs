use rocket::Responder;
use rocket::serde::Serialize;

#[derive(Debug, Serialize, Responder)]
pub struct MessageResponse {
    pub message: String,
}

