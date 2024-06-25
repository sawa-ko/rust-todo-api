use rocket::{FromForm, post};
use rocket::form::Form;
use rocket::form::validate::msg;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use sea_orm_rocket::Connection;
use serde::Deserialize;
use database::Db;
use database::entities::user as User;
use services::user::mutations::user::{SignIn, UserMutations};
use crate::routes::ResponseRequest;

#[derive(Serialize, Deserialize, FromForm)]
pub struct SignInPayload {
    #[field(validate = len(5..).or_else(msg!("The description must be at least 5 characters long.")))]
    pub username: String,
    #[field(validate = len(5..).or_else(msg!("The description must be at least 6 characters long.")))]
    pub password: String,
}

#[post("/sign-in", data = "<payload>")]
pub async fn sign_in(payload: Form<SignInPayload>, conn: Connection<'_, Db>) -> Custom<Json<ResponseRequest<Option<SignIn>>>> {
    let db = conn.into_inner();
    let payload = payload.into_inner();
    let sign_in_result = UserMutations::sign_in(payload.username, payload.password, db).await;
    
    match sign_in_result { 
        Ok(sign_in) => Custom(Status::Ok, Json(ResponseRequest {
            status: 200,
            message: Some("Sign in successful".to_string()),
            data: Some(sign_in),
        })),
        Err(e) => Custom(Status::Unauthorized, Json(ResponseRequest {
            status: 401,
            message: Some(e.to_string()),
            data: None,
        }))
    }
}
#[post("/sign-up", data = "<payload>")]
pub async fn sign_up(payload: Form<SignInPayload>, conn: Connection<'_, Db>) -> Custom<Json<ResponseRequest<Option<User::Model>>>> {
    let db = conn.into_inner();
    let payload = payload.into_inner();
    let sign_up_result = UserMutations::create(payload.username, payload.password, db).await;
    
    match sign_up_result { 
        Ok(sign_up) => Custom(Status::Ok, Json(ResponseRequest {
            status: 200,
            message: Some("Sign up successful".to_string()),
            data: Some(sign_up),
        })),
        Err(e) => Custom(Status::Unauthorized, Json(ResponseRequest {
            status: 401,
            message: Some(e.to_string()),
            data: None,
        }))
    }
}