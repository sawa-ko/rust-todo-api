use serde::Serialize;

pub mod ping;
pub mod task;

#[derive(Serialize)]
pub struct ResponseRequest<T> {
    message: Option<String>,
    status: u16,
    data: T,
}
