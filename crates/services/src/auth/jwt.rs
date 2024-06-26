use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};
use std::env;
use std::ops::Add;
use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

impl JWT {
    pub fn encode(id: &i32) -> Result<String, Error> {
        dotenvy::dotenv().expect("Error loading .env file!");
        let secret = env::var("JWT_SECRET")
            .expect("Error creating auth token: JWT_SECRET environment variable is missing");
        let exp = SystemTime::now().add(Duration::from_secs(3600));
        let claims = Claims {
            sub: *id,
            exp: exp
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize,
        };

        let header = Header::new(Algorithm::HS512);
        jsonwebtoken::encode(
            &header,
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    pub fn decode(token_data: String) -> Result<TokenData<Claims>, ErrorKind> {
        dotenvy::dotenv().expect("Error loading .env file!");
        let secret = env::var("JWT_SECRET");
        let token = token_data.trim_start_matches("Bearer").trim();

        match jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.unwrap().as_bytes()),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(token) => Ok(token),
            Err(e) => Err(e.kind().to_owned()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            None => Outcome::Error((
                rocket::http::Status::Unauthorized,
                "No auth token provided".to_string(),
            )),
            Some(v) => match JWT::decode(v.to_string()) {
                Ok(token) => Outcome::Success(JWT {
                    claims: token.claims,
                }),
                Err(e) => match &e {
                    ErrorKind::ExpiredSignature => Outcome::Error((
                        rocket::http::Status::Unauthorized,
                        "Token has expired".to_string(),
                    )),
                    ErrorKind::InvalidToken => Outcome::Error((
                        rocket::http::Status::Unauthorized,
                        "Invalid user auth token.".to_string(),
                    )),
                    _ => Outcome::Error((
                        rocket::http::Status::Unauthorized,
                        "Ah error occurred when received the auth token.".to_string(),
                    )),
                },
            },
        }
    }
}
