use crate::routes::{Response, ResponseRequest};
use database::entities::user as User;
use database::Db;
use rocket::form::validate::msg;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::{get, post, FromForm};
use sea_orm_rocket::Connection;
use services::auth::jwt::JWT;
use services::user::models::user::UserModel;
use services::user::mutations::user::{SignIn, UserMutations};
use services::user::queries::user::UserQueries;

#[derive(Serialize, Deserialize, FromForm)]
pub struct SignInPayload {
    #[field(validate = len(5..).or_else(msg!("The description must be at least 5 characters long.")))]
    pub username: String,
    #[field(validate = len(5..).or_else(msg!("The description must be at least 6 characters long.")))]
    pub password: String,
}

#[post("/sign-in", data = "<payload>")]
pub async fn sign_in(
    payload: Form<SignInPayload>,
    conn: Connection<'_, Db>,
) -> Response<Option<SignIn>> {
    let db = conn.into_inner();
    let payload = payload.into_inner();
    let sign_in_result = UserMutations::sign_in(payload.username, payload.password, db).await;

    match sign_in_result {
        Ok(sign_in) => Custom(
            Status::Ok,
            Json(ResponseRequest {
                status: Status::Ok,
                message: Some("Sign in successful".to_string()),
                data: Some(sign_in),
            }),
        ),
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

#[post("/sign-up", data = "<payload>")]
pub async fn sign_up(
    payload: Form<SignInPayload>,
    conn: Connection<'_, Db>,
) -> Response<Option<User::Model>> {
    let db = conn.into_inner();
    let payload = payload.into_inner();
    let sign_up_result = UserMutations::create(payload.username, payload.password, db).await;

    match sign_up_result {
        Ok(sign_up) => Custom(
            Status::Ok,
            Json(ResponseRequest {
                status: Status::Ok,
                message: Some("Sign up successful".to_string()),
                data: Some(sign_up),
            }),
        ),
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

#[get("/me")]
pub async fn me(
    user: JWT,
    conn: Connection<'_, Db>,
) -> Response<Option<UserModel>> {
    let db = conn.into_inner();
    let user = UserQueries::get_current_user(user, db).await;

    match user {
        Ok(u) => Custom(
            Status::Ok,
            Json(ResponseRequest {
                status: Status::Ok,
                message: Some("Sign up successful".to_string()),
                data: Some(u),
            }),
        ),
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
