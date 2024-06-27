use crate::routes::{Response, ResponseRequest};
use database::entities::user as User;
use database::Db;
use rocket::form::validate::msg;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, FromForm};
use sea_orm_rocket::Connection;
use services::auth::jwt::JWT;
use services::user::models::user::UserModel;
use services::user::mutations::user::{SignIn, UserMutations};
use services::user::queries::user::UserQueries;

/// Payload structure for signing in a user.
#[derive(Serialize, Deserialize, FromForm)]
pub struct SignInPayload {
    /// Username input for sign-in.
    #[field(validate = len(5..).or_else(msg!("The username must be at least 5 characters long.")))]
    pub username: String,
    /// Password input for sign-in.
    #[field(validate = len(5..).or_else(msg!("The password must be at least 5 characters long.")))]
    pub password: String,
}

/// Endpoint for user sign-in.
///
/// This function handles the HTTP POST request to authenticate and sign in a user.
/// It expects a JSON payload `SignInPayload` containing username and password.
///
/// # Arguments
///
/// * `payload` - JSON payload containing `SignInPayload` data.
/// * `conn` - SeaORM database connection (`Connection<'_, Db>`).
///
/// # Returns
///
/// A custom response (`Response<Option<SignIn>>`) with status `200 OK` on success or `401 Unauthorized` on failure.
///
#[post("/sign-in", data = "<payload>")]
pub async fn sign_in(
    payload: Form<SignInPayload>,
    conn: Connection<'_, Db>,
) -> Response<Option<SignIn>> {
    // Extract database connection and payload data
    let db = conn.into_inner();

    // Extract payload data
    let payload = payload.into_inner();

    // Attempt to sign in the user using provided credentials
    let sign_in_result = UserMutations::sign_in(payload.username, payload.password, db).await;

    match sign_in_result {
        // Return a successful response with sign-in details
        Ok(sign_in) => Custom(
            Status::Ok,
            Json(ResponseRequest {
                status: Status::Ok,
                message: Some("Sign in successful".to_string()),
                data: Some(sign_in),
            }),
        ),
        // Return an unauthorized response with the error message
        Err(e) => Custom(
            Status::Unauthorized,
            Json(ResponseRequest {
                status: Status::Unauthorized,
                message: Some(e.to_string()),
                data: None,
            }),
        ),
    }
}

/// Endpoint for user sign-up.
///
/// This function handles the HTTP POST request to create a new user account.
/// It expects a JSON payload `SignInPayload` containing username and password.
///
/// # Arguments
///
/// * `payload` - JSON payload containing `SignInPayload` data.
/// * `conn` - SeaORM database connection (`Connection<'_, Db>`).
///
/// # Returns
///
/// A custom response (`Response<Option<User::Model>>`) with status `200 OK` on success or `401 Unauthorized` on failure.
///
#[post("/sign-up", data = "<payload>")]
pub async fn sign_up(
    payload: Form<SignInPayload>,
    conn: Connection<'_, Db>,
) -> Response<Option<User::Model>> {
    // Extract database connection and payload data
    let db = conn.into_inner();

    // Extract payload data
    let payload = payload.into_inner();

    // Attempt to create a new user account using provided credentials
    let sign_up_result = UserMutations::create(payload.username, payload.password, db).await;

    match sign_up_result {
        // Return a successful response with sign-up details
        Ok(sign_up) => Custom(
            Status::Ok,
            Json(ResponseRequest {
                status: Status::Ok,
                message: Some("Sign up successful".to_string()),
                data: Some(sign_up),
            }),
        ),
        // Return an unauthorized response with the error message
        Err(e) => Custom(
            Status::Unauthorized,
            Json(ResponseRequest {
                status: Status::Unauthorized,
                message: Some(e.to_string()),
                data: None,
            }),
        ),
    }
}

/// Endpoint to fetch current user information.
///
/// This function handles the HTTP GET request to retrieve information about the current authenticated user.
/// It expects a valid JWT (`JWT`) as part of the request headers for authentication.
///
/// # Arguments
///
/// * `user` - JWT containing user claims.
/// * `conn` - SeaORM database connection (`Connection<'_, Db>`).
///
/// # Returns
///
/// A custom response (`Response<Option<UserModel>>`) with status `200 OK` on success or `401 Unauthorized` on failure.
///
#[get("/me")]
pub async fn me(user: JWT, conn: Connection<'_, Db>) -> Response<Option<UserModel>> {
    // Extract database connection
    let db = conn.into_inner();

    // Retrieve current user information using the provided JWT
    let user = UserQueries::get_current_user(user, db).await;

    match user {
        // Return a successful response with user details
        Ok(u) => Custom(
            Status::Ok,
            Json(ResponseRequest {
                status: Status::Ok,
                message: Some("Sign up successful".to_string()),
                data: Some(u),
            }),
        ),
        // Return an unauthorized response with the error message
        Err(e) => Custom(
            Status::Unauthorized,
            Json(ResponseRequest {
                status: Status::Unauthorized,
                message: Some(e.to_string()),
                data: None,
            }),
        ),
    }
}
