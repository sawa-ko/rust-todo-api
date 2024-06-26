use rocket::serde::{Deserialize, Serialize};

pub mod auth;
pub mod ping;
pub mod task;

#[derive(Serialize, Deserialize)]
pub struct ResponseRequest<T> {
    message: Option<String>,
    status: u16,
    data: T,
}
