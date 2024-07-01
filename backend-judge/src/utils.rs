use chrono::NaiveDateTime;
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::responses::MessageResponse;


pub fn status_message(status:Status, message: &str) -> (Status, Json<MessageResponse>){
    (status, Json(MessageResponse { message: message.to_string() }))
}


pub fn get_current_ist() ->NaiveDateTime{
    let utc_time = chrono::Utc::now();
    let ist_time = utc_time.with_timezone(&chrono::FixedOffset::east(5*3600 + 30*60));
    ist_time.naive_local()
}