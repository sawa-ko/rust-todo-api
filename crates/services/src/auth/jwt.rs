use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{Duration, SystemTime};

/// Constant defining the encryption algorithm used for JWT tokens.
pub const JWT_ALGORITHM: Algorithm = Algorithm::HS512;

/// Duration in seconds for the generated JWT token's validity period.
pub const TOKEN_DURATION_SECS: u64 = 3600;

/// Struct representing the claims contained within the JWT token.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject identifier within the JWT claims.
    pub sub: i32,
    /// Expiration time of the JWT token in UNIX timestamp (seconds since epoch).
    pub exp: usize,
}

/// Struct representing a decoded JWT token.
#[derive(Debug)]
pub struct JWT {
    /// Claims parsed from the JWT token.
    pub claims: Claims,
}

impl JWT {
    /// Encode a JWT token with specified user ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The user ID to include in the JWT claims.
    ///
    /// # Returns
    ///
    /// A Result containing the encoded JWT token as a String or an Error.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - Loading the `.env` file fails.
    /// - Retrieving the JWT secret from environment variables fails.
    /// - Calculating the token expiration time fails.
    pub fn encode(id: &i32) -> Result<String, Error> {
        // Load environment variables from `.env` file.
        if dotenvy::dotenv().is_err() {
            println!("Error loading .env file!");
        }

        // Retrieve JWT secret from environment variables.
        let secret = env::var("JWT_SECRET")
            .expect("Error creating auth token: JWT_SECRET environment variable is missing");

        // Calculate token expiration time.
        let exp = SystemTime::now().checked_add(Duration::from_secs(TOKEN_DURATION_SECS)).expect("Failed to calculate token expiration time");

        // Construct JWT claims.
        let claims = Claims {
            sub: *id,
            exp: exp
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize,
        };

        // Construct JWT header with specified algorithm.
        let header = Header::new(Algorithm::HS512);

        // Encode JWT token using claims and secret key.
        jsonwebtoken::encode(
            &header,
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    /// Decode a JWT token and validate its authenticity.
    ///
    /// # Arguments
    ///
    /// * `token_data` - The JWT token string to decode.
    ///
    /// # Returns
    ///
    /// A Result containing the decoded TokenData or an ErrorKind.
    ///
    /// # Errors
    ///
    /// This function can fail if:
    /// - Loading the `.env` file fails.
    /// - Retrieving the JWT secret from environment variables fails.
    /// - Decoding the JWT token fails due to expiration or invalidity.
    pub fn decode(token_data: String) -> Result<TokenData<Claims>, ErrorKind> {
        // Load environment variables from `.env` file.
        if dotenvy::dotenv().is_err() {
            println!("Error loading .env file!");
        }

        // Retrieve JWT secret from environment variables.
        let secret = env::var("JWT_SECRET");

        // Trim 'Bearer ' prefix and any leading/trailing whitespace from token data.
        let token = token_data.trim_start_matches("Bearer").trim();

        // Decode JWT token with specified secret and algorithm validation.
        match jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.unwrap().as_bytes()),
            &Validation::new(JWT_ALGORITHM),
        ) {
            Ok(token) => Ok(token),
            Err(e) => Err(e.kind().to_owned()),
        }
    }
}

/// Implementation of Rocket's `FromRequest` trait for JWT authentication.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = String;

    /// Performs authentication and authorization checks for incoming requests.
    ///
    /// # Arguments
    ///
    /// * `request` - The incoming Rocket request.
    ///
    /// # Returns
    ///
    /// An `Outcome` containing either a validated JWT instance or an error message.
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Check if Authorization header is present in the request.
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
