use rocket::serde::{Deserialize, Serialize};

pub mod ping;
pub mod task;
pub mod auth;

#[derive(Serialize, Deserialize)]
pub struct ResponseRequest<T> {
    message: Option<String>,
    status: u16,
    data: T,
}
