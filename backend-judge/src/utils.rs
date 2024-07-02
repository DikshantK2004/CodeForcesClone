use chrono::NaiveDateTime;
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::responses::MessageResponse;


pub fn status_message(status:Status, message: &str) -> (Status, Json<MessageResponse>){
    (status, Json(MessageResponse { message: message.to_string() }))
}


pub fn get_current_utc() ->NaiveDateTime{
    let utc_time = chrono::Utc::now();
    utc_time.naive_local()
}