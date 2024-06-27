use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

pub mod auth;
pub mod ping;
pub mod task;

#[derive(Serialize, Deserialize)]
pub struct ResponseRequest<T> {
    message: Option<String>,
    status: Status,
    data: T,
}

pub type Response<T> = Custom<Json<ResponseRequest<T>>>;
