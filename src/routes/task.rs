use database::entities::task as Task;
use database::Db;
use rocket::form::{Error, Form};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{delete, form, get, patch, post, FromForm};
use sea_orm_rocket::Connection;
use services::task::mutations::task::{TaskMutation, TaskPayload};
use services::task::queries::task::{GetAllTasks, PaginationPayload, TaskQueries};
use crate::routes::ResponseRequest;

#[derive(Deserialize, Serialize, FromForm)]
pub struct ManageTodo {
    #[field(validate = len(2..=20).or_else(msg!("The name must be between 2 to 10 characters long.")))]
    name: String,
    #[field(validate = len(5..=200).or_else(msg!("The description must be at least 5 characters long.")))]
    description: String,
    #[field(default = false)]
    is_active: bool,
}

#[post("/create", data = "<form>")]
pub async fn create_task(
    form: Form<ManageTodo>,
    conn: Connection<'_, Db>,
) -> Json<ResponseRequest<Option<Task::Model>>> {
    let db = conn.into_inner();
    let todo = form.into_inner();

    let task = TaskMutation::create(TaskPayload {
        name: todo.name.trim().to_owned(),
        description: todo.description.trim().to_owned(),
        is_active: todo.is_active,
    }, db).await;
    
    if let Err(_) = task {
        let res = ResponseRequest {
            message: Some("Failed to create task".to_string()),
            status: 500,
            data: None,
        };

        return Json(res);
    }
    
    let res = ResponseRequest {
        message: Some("Task created successfully".to_string()),
        status: 200,
        data: Some(task.unwrap()),
    };
    
    Json(res)
}

#[patch("/update/<id>", data = "<form>")]
pub async fn update_task(
    form: Form<ManageTodo>,
    id: i32,
    conn: Connection<'_, Db>,
) -> Json<ResponseRequest<Option<Task::Model>>> {
    let db = conn.into_inner();
    let todo = form.into_inner();
    
    let task = TaskMutation::update(TaskPayload {
        name: todo.name.trim().to_owned(),
        description: todo.description.trim().to_owned(),
        is_active: todo.is_active,
    }, id, db).await;
    
    if let Err(_) = task {
        let res = ResponseRequest {
            message: Some("Failed to update task".to_string()),
            status: 500,
            data: None,
        };

        return Json(res);
    }
    
    let res = ResponseRequest {
        message: Some("Task updated successfully".to_string()),
        status: 200,
        data: Some(task.unwrap()),
    };
    
    Json(res)
}

#[delete("/delete/<id>")]
pub async fn delete_task(
    id: i32,
    conn: Connection<'_, Db>,
) -> Json<ResponseRequest<Option<Task::Model>>> {
    let db = conn.into_inner();
    let result = TaskMutation::delete(id, db).await;
    
    if let Err(_) = result {
        let res = ResponseRequest {
            message: Some("Failed to delete task".to_string()),
            status: 500,
            data: None,
        };

        return Json(res);
    }
    
    let res = ResponseRequest {
        message: Some("Task deleted successfully".to_string()),
        status: 200,
        data: None,
    };
    
    Json(res)
}

#[derive(FromForm, Serialize)]
pub struct FilterTasks {
    #[field(default = Some(1), validate = validate_min_params(String::from("page")))]
    page: Option<i32>,
    #[field(default = Some(10), validate = validate_min_params(String::from("size")))]
    size: Option<i32>,
    query: Option<String>,
}

fn validate_min_params<'v>(value: &Option<i32>, field_name: String) -> form::Result<'v, ()> {
    if let Some(val) = value {
        if *val < 1 {
            Err(Error::validation(format!(
                "The {} number must be greater than 0.",
                field_name
            )))?;
        }
    }

    Ok(())
}

#[get("/?<filter..>")]
pub async fn get_tasks(filter: FilterTasks, conn: Connection<'_, Db>) -> Json<ResponseRequest<Option<GetAllTasks>>> {
    let payload = PaginationPayload {
        page: filter.page.unwrap_or(1) as u64,
        size: filter.size.unwrap_or(10) as u64,
        query: filter.query.clone(),
    };

    let db = conn.into_inner();
    let tasks = TaskQueries::get_tasks(payload, db).await;
    
    if let Err(_) = tasks {
        let res = ResponseRequest {
            message: Some("Failed to fetch tasks".to_string()),
            status: 500,
            data: None,
        };

        return Json(res);
    }
    
    let res = ResponseRequest {
        message: None,
        status: 200,
        data: Some(tasks.unwrap()),
    };

    Json(res)
}
#[get("/<id>")]
pub async fn get_task(
    id: i32,
    conn: Connection<'_, Db>,
) -> Json<ResponseRequest<Option<Task::Model>>> {
    let db = conn.into_inner();
    let result = TaskQueries::get_task_by_id(id, db).await;

    if let Err(_) = result {
        let res = ResponseRequest {
            message: Some("The task was not found".to_string()),
            status: 404,
            data: None,
        };

        return Json(res);
    }

    let res = ResponseRequest {
        message: None,
        status: 200,
        data: Some(result.unwrap()),
    };

    Json(res)
}