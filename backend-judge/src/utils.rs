use rocket::http::Status;
use rocket::serde::json::Json;
use crate::responses::MessageResponse;


pub fn status_message(status:Status, message: &str) -> (Status, Json<MessageResponse>){
    (status, Json(MessageResponse { message: message.to_string() }))
}
