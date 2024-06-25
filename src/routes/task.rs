use rocket::{delete, form, FromForm, get, patch, post};
use rocket::form::{Error, Form};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm_rocket::Connection;
use database::Db;
use database::entities::{task as Task};

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

#[post("/create", data = "<form>")]
pub async fn create_task(form: Form<ManageTodo>, conn: Connection<'_, Db>) -> Json<ResponseRequest<Option<Task::Model>>> {
    let db = conn.into_inner();
    let todo = form.into_inner();
    
    let task = Task::ActiveModel {
        id: Default::default(),
        name: Set(todo.name.trim().to_owned()),
        description: Set(todo.description.trim().to_owned()),
        is_active: Set(todo.is_complete.to_owned()),
    };
    
    let task_result = Task::Entity::insert(task).exec(db).await;
    
    if task_result.is_err() {
        let res = ResponseRequest {
          message: Some("Failed to create task".to_string()),
          status: 500,
          data: None
        };
        
        return Json(res);
    }

    let new_task = Task::Entity::find_by_id(task_result.unwrap().last_insert_id).one(db).await;
    
    let res = ResponseRequest {
        message: Some("Task created successfully".to_string()),
        status: 200,
        data: new_task.unwrap()
    };

    Json(res)
}

#[patch("/update/<id>", data = "<form>")]
pub async fn update_task(form: Form<ManageTodo>, id: i32, conn: Connection<'_, Db>) -> Json<ResponseRequest<Option<Task::Model>>> {
    let db = conn.into_inner();
    let task_query = Task::Entity::find_by_id(id).one(db).await;
    
    if task_query.unwrap_or(None).is_none() {
        let res = ResponseRequest {
            message: Some("Task not found".to_string()),
            status: 404,
            data: None
        };
        
        return Json(res);
    }
    
    let todo = form.into_inner();
    let task = Task::ActiveModel {
        id: Set(id.to_owned()),
        name: Set(todo.name.trim().to_owned()),
        description: Set(todo.description.trim().to_owned()),
        is_active: Set(todo.is_complete.to_owned()),
    };
    
    let task_result = task.update(db).await;
    if task_result.is_err() {
        let res = ResponseRequest {
            message: Some("Failed to update task".to_string()),
            status: 500,
            data: None
        };
        
        return Json(res);
    }
    
    let updated_task = Task::Entity::find_by_id(id).one(db).await;
    let res = ResponseRequest {
        message: Some("Task updated successfully".to_string()),
        status: 200,
        data: updated_task.unwrap()
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