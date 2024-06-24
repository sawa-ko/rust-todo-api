use serde::Serialize;

pub mod task;
pub mod ping;

#[derive(Serialize)]
pub struct ResponseRequest<T> {
    message: Option<String>,
    status: u16,
    data: T
}