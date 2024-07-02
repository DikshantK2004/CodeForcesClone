use chrono::Utc;
use pwhash::bcrypt;

pub fn hash_password(password: &str) -> String {
    bcrypt::hash(password).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash)
}


use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::errors::{ ErrorKind};
use rocket::futures::TryFutureExt;
use rocket::request::FromRequest;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: usize,
}


pub struct AuthenticatedUser(pub(crate) i32);

#[derive(Debug)]
pub enum AuthError{
    NoToken,
    InvalidToken,
    ExpiredToken,
}

use std::fmt;

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthError::NoToken => write!(f, "No token provided."),
            AuthError::InvalidToken => write!(f, "Token is invalid."),
            AuthError::ExpiredToken => write!(f, "Token has expired."),
        }
    }
}


#[rocket::async_trait]
impl<'r>  FromRequest<'r> for AuthenticatedUser {
    type Error = AuthError;

    async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let token = request.cookies().get("token").map(|cookie| cookie.value());
        if token.is_none() {
            return rocket::request::Outcome::Error((rocket::http::Status::Unauthorized, AuthError::NoToken));
        }

        let token = token.unwrap();

        let decoded = decode_jwt(&token);
        match decoded {
            Ok(claims) => {
                println!("User id: {:?}", claims.user_id);
                println!("Token expires at: {:?}", claims.exp);
                rocket::request::Outcome::Success(AuthenticatedUser(claims.user_id))
            },
            Err(e) => rocket::request::Outcome::Error((rocket::http::Status::Unauthorized,  e)),
        }
    }
}

pub fn create_jwt(user_id: i32) -> Result<String, String>{
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(5));

    if let None = expiration {
        return Err("Could not create the token".to_string());
    }

    let expiration = expiration.unwrap().timestamp();
    let claims = Claims {
        user_id,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret("secret".as_ref()))
        .map_err(|_| "Could not create the token".to_string())
}

pub fn decode_jwt(token: &str) -> Result<Claims, AuthError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS512),
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => match *err.kind() {
            ErrorKind::ExpiredSignature => Err(AuthError::ExpiredToken),
            _ => Err(AuthError::InvalidToken),
        }
    }
}