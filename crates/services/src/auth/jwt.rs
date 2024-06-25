use std::env;
use std::ops::Add;
use std::time::{Duration, SystemTime};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use jsonwebtoken::errors::{Error, ErrorKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

pub struct JWT {
    pub claims: Claims
}

impl JWT {
    pub fn encode(id: &i32) -> Result<String, Error> {
        dotenvy::dotenv().expect("Error loading .env file!");
        let secret = env::var("JWT_SECRET");
        let exp = SystemTime::now().add(Duration::from_secs(3600));
        let claims = Claims {
            sub: *id,
            exp: exp.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as usize
        };
        
        let header = Header::new(Algorithm::HS512);
        jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(secret.unwrap().as_bytes()))
    }
    
    pub fn decode(token_data: String) -> Result<TokenData<Claims>, ErrorKind> {
        dotenvy::dotenv().expect("Error loading .env file!");
        let secret = env::var("JWT_SECRET");
        let token = token_data.trim_start_matches("Bearer").trim();
        
        match jsonwebtoken::decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.unwrap().as_bytes()),
            &Validation::new(Algorithm::HS512)
        ) { 
            Ok(token) => Ok(token),
            Err(e) => Err(e.kind().to_owned())
        }
    }
}