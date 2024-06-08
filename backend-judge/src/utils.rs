use pwhash::bcrypt;
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::responses::MessageResponse;


pub fn hash_password(password: &str) -> String {
    bcrypt::hash(password).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash)
}

pub fn status_message(status:Status, message:&str) -> (Status, Json<MessageResponse>){
    (status, Json(MessageResponse { message: message.to_string() }))
}
