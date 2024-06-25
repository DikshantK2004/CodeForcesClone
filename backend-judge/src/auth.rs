use chrono::Utc;
use pwhash::bcrypt;
use crate::models::Claims;

pub fn hash_password(password: &str) -> String {
    bcrypt::hash(password).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash)
}


use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::errors::{ ErrorKind};
use rocket::futures::TryFutureExt;


pub fn create_jwt(email:&str) -> Result<String, String>{
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(5))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        email: email.to_string(),
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret("secret".as_ref()))
        .map_err(|_| "Could not create the token".to_string())
}

pub fn decode_jwt(token: &str) -> Result<Claims, String> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS512),
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => match *err.kind() {
            ErrorKind::ExpiredSignature => Err("Token Expired".to_string()),
            _ => Err("Could not decode the token".to_string()),
        }
    }
}