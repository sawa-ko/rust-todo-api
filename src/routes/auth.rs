use rocket::{FromForm, post};
use rocket::form::Form;
use rocket::form::validate::msg;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use serde::Deserialize;
use crate::routes::ResponseRequest;

#[derive(Serialize, Deserialize, FromForm)]
pub struct SignInPayload {
    #[field(validate = len(5..).or_else(msg!("The description must be at least 5 characters long.")))]
    pub username: String,
    #[field(validate = len(5..).or_else(msg!("The description must be at least 6 characters long.")))]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInResponse {
    pub token: String,
    pub user_id: i32,
}

#[post("/sign-in", data = "<payload>")]
pub async fn sign_in(payload: Form<SignInPayload>) -> Json<ResponseRequest<SignInResponse>> {
    let result = ResponseRequest {
        status: 200,
        message: None,
        data: SignInResponse {
            user_id: 1,
            token: "token".to_string(),
        }
    };
    
    Json(result)
}