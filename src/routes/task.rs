use rocket::{delete, form, FromForm, get, patch, post};
use rocket::form::{Error, Form};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

use crate::routes::ResponseRequest;

#[derive(Deserialize, Serialize, FromForm)]
pub struct ManageTodo {
    #[field(validate = len(2..=20).or_else(msg!("The name must be between 2 to 10 characters long.")))]
    name: String,
    #[field(validate = len(5..=200).or_else(msg!("The description must be at least 5 characters long.")))]
    description: String,
    #[field(default = false)]
    is_complete: bool
}

#[post("/create", data = "<todo>")]
pub async fn create_task(todo: Form<ManageTodo>) -> Json<ResponseRequest<ManageTodo>> {
    let res = ResponseRequest {
        message: Some("Task created successfully".to_string()),
        status: 200,
        data: todo.into_inner()
    };

    Json(res)
}

#[patch("/update/<id>", data = "<todo>")]
pub fn update_task(todo: Form<ManageTodo>, id: i32) -> Json<ResponseRequest<ManageTodo>> {
    println!("{}", &id);

    let res = ResponseRequest {
        message: Some("Task updated successfully".to_string()),
        status: 200,
        data: todo.into_inner()
    };

    Json(res)
}

#[delete("/delete/<id>")]
pub fn delete_task(id: i32) -> Json<ResponseRequest<i32>> {
    let res = ResponseRequest {
        message: Some("Task deleted successfully".to_string()),
        status: 200,
        data: id
    };

    Json(res)
}

#[derive(FromForm, Serialize)]
pub struct FilterTasks {
    #[field(default = Some(1), validate = validate_min_params(String::from("page")))]
    page: Option<i32>,
    #[field(default = Some(10), validate = validate_min_params(String::from("size")))]
    size: Option<i32>,
    query: Option<String>
}

fn validate_min_params<'v>(value: &Option<i32>, field_name: String) -> form::Result<'v, ()> {
    if let Some(val) = value {
        if *val < 1 {
            Err(Error::validation(format!("The {} number must be greater than 0.", field_name)))?;
        }
    }

    Ok(())
}

#[get("/?<filter..>")]
pub fn get_tasks(filter: FilterTasks) -> Json<ResponseRequest<FilterTasks>> {
    let res = ResponseRequest {
        message: None,
        status: 200,
        data: filter
    };

    Json(res)
}