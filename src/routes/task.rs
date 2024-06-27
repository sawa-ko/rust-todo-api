use crate::routes::{Response, ResponseRequest};
use database::entities::task as Task;
use database::Db;
use rocket::form::{Error, Form};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{delete, form, get, patch, post, FromForm};
use sea_orm_rocket::Connection;
use services::auth::jwt::JWT;
use services::task::models::task::TaskModel;
use services::task::mutations::task::{TaskMutation, TaskPayload};
use services::task::queries::task::{GetAllTasks, PaginationPayload, TaskQueries};

/// Struct representing the payload for managing a TODO task.
///
/// This struct is used for handling form data and validating the input for creating or updating a task.
///
#[derive(Deserialize, Serialize, FromForm)]
pub struct ManageTodo {
    /// The name of the task.
    #[field(validate = len(2..=20).or_else(msg!("The name must be between 2 to 10 characters long.")))]
    pub name: String,
    /// The description of the task.
    #[field(validate = len(5..=200).or_else(msg!("The description must be at least 5 characters long.")))]
    pub description: String,
    /// Flag indicating whether the task is active or not. Defaults to `false`.
    #[field(default = false)]
    pub is_active: bool,
}

/// Endpoint for creating a new task.
///
/// This function handles the HTTP POST request to create a new task.
/// It expects a JSON payload `ManageTodo` containing task details.
///
/// # Arguments
///
/// * `form` - JSON payload containing `ManageTodo` data.
/// * `user` - JWT token representing the authenticated user.
/// * `conn` - SeaORM database connection (`Connection<'_, Db>`).
///
/// # Returns
///
/// A custom response (`Response<Option<Task::Model>>`) with status `200 OK` on success or `500 Internal Server Error` on failure.
///
#[post("/create", data = "<form>")]
pub async fn create_task(
    form: Form<ManageTodo>,
    user: JWT,
    conn: Connection<'_, Db>,
) -> Response<Option<Task::Model>> {
    // Extract database connection
    let db = conn.into_inner();
    
    // Extract payload data
    let todo = form.into_inner();

    // Attempt to create a new task using provided payload
    let task = TaskMutation::create(
        TaskPayload {
            name: todo.name.trim().to_owned(),
            description: todo.description.trim().to_owned(),
            is_active: todo.is_active,
            user_id: user.claims.sub,
        },
        db,
    )
    .await;

    match task {
        // Return a successful response with the created task details
        Ok(created_task) => Custom(
            Status::Ok,
            Json(ResponseRequest {
                message: Some("Task created successfully".to_string()),
                status: Status::Ok,
                data: Some(created_task),
            }),
        ),
        // Return an internal server error response with the error message
        Err(_) => Custom(
            Status::InternalServerError,
            Json(ResponseRequest {
                message: Some("Failed to create task".to_string()),
                status: Status::InternalServerError,
                data: None,
            }),
        ),
    }
}

/// Endpoint for updating an existing task.
///
/// This function handles the HTTP PATCH request to update an existing task identified by `id`.
/// It expects a JSON payload `ManageTodo` containing updated task details.
///
/// # Arguments
///
/// * `form` - JSON payload containing `ManageTodo` data.
/// * `user` - JWT token representing the authenticated user.
/// * `id` - The ID of the task to be updated.
/// * `conn` - SeaORM database connection (`Connection<'_, Db>`).
///
/// # Returns
///
/// A custom response (`Response<Option<Task::Model>>`) with status `200 OK` on success or `500 Internal Server Error` on failure.
///
#[patch("/update/<id>", data = "<form>")]
pub async fn update_task(
    form: Form<ManageTodo>,
    user: JWT,
    id: i32,
    conn: Connection<'_, Db>,
) -> Response<Option<Task::Model>> {
    // Extract database connection
    let db = conn.into_inner();

    // Extract payload data
    let todo = form.into_inner();

    // Attempt to update an existing task using provided payload
    let task = TaskMutation::update(
        TaskPayload {
            name: todo.name.trim().to_owned(),
            description: todo.description.trim().to_owned(),
            is_active: todo.is_active,
            user_id: user.claims.sub,
        },
        id,
        db,
    )
    .await;

    match task {
        // Return a successful response with the updated task details
        Ok(updated_task) => Custom(
            Status::Ok,
            Json(ResponseRequest {
                message: Some("Task created successfully".to_string()),
                status: Status::Ok,
                data: Some(updated_task),
            }),
        ),
        // Return an internal server error response with the error message
        Err(_) => Custom(
            Status::InternalServerError,
            Json(ResponseRequest {
                message: Some("Failed to update task".to_string()),
                status: Status::InternalServerError,
                data: None,
            }),
        ),
    }
}

/// Endpoint for deleting a task.
///
/// This function handles the HTTP DELETE request to delete a task identified by `id`.
///
/// # Arguments
///
/// * `id` - The ID of the task to be deleted.
/// * `user` - JWT token representing the authenticated user.
/// * `conn` - SeaORM database connection (`Connection<'_, Db>`).
///
/// # Returns
///
/// A custom response (`Response<u64>`) with status `200 OK` on success or `500 Internal Server Error` on failure.
///
#[delete("/delete/<id>")]
pub async fn delete_task(
    id: i32,
    user: JWT,
    conn: Connection<'_, Db>,
) -> Response<u64> {
    // Extract database connection
    let db = conn.into_inner();
    
    // Attempt to delete the task using provided ID
    let result = TaskMutation::delete(id, user.claims.sub, db).await;

    match result {
        // Return a successful response with the number of rows affected
        Ok(deleted_task) => Custom(
            Status::Ok,
            Json(ResponseRequest {
                message: Some("Task created successfully".to_string()),
                status: Status::Ok,
                data: deleted_task.rows_affected,
            }),
        ),
        // Return an internal server error response with the error message
        Err(_) => Custom(
            Status::InternalServerError,
            Json(ResponseRequest {
                message: Some("Failed to delete the task".to_string()),
                status: Status::InternalServerError,
                data: 0,
            }),
        ),
    }
}

/// Struct representing the filters for querying tasks.
///
/// This struct is used for handling query parameters and validating them for fetching tasks.
///
#[derive(FromForm, Serialize)]
pub struct FilterTasks {
    /// The page number for pagination. Defaults to `1`.
    #[field(default = Some(1), validate = validate_min_params(String::from("page")))]
    pub page: Option<i32>,
    /// The number of tasks per page. Defaults to `10`.
    #[field(default = Some(10), validate = validate_min_params(String::from("size")))]
    pub size: Option<i32>,
    /// Optional query string for filtering tasks by name or description.
    pub query: Option<String>,
}

/// Validates that the provided value is greater than 0.
///
/// This function is used to validate page and size parameters in `FilterTasks`.
///
/// # Arguments
///
/// * `value` - Reference to the value to be validated (`Option<i32>`).
/// * `field_name` - Name of the field being validated (e.g., "page", "size").
///
/// # Returns
///
/// A `form::Result` indicating success or a validation error.
///
fn validate_min_params<'v>(value: &Option<i32>, field_name: String) -> form::Result<'v, ()> {
    // Check if the value is less than 1
    if let Some(val) = value {
        // Return an error if the value is less than 1
        if *val < 1 {
            // Return a validation error with a custom message
            Err(Error::validation(format!(
                "The {} number must be greater than 0.",
                field_name
            )))?;
        }
    }

    // Return success if the value is valid
    Ok(())
}

/// Endpoint for fetching a list of tasks.
///
/// This function handles the HTTP GET request to fetch a list of tasks based on optional filters.
/// It accepts query parameters `page`, `size`, and `query` to paginate and filter tasks.
///
/// # Arguments
///
/// * `filter` - Struct containing pagination and filtering parameters (`FilterTasks`).
/// * `user` - JWT token representing the authenticated user.
/// * `conn` - SeaORM database connection (`Connection<'_, Db>`).
///
/// # Returns
///
/// A custom response (`Response<Option<GetAllTasks>>`) with status `200 OK` on success or `500 Internal Server Error` on failure.
///
#[get("/?<filter..>")]
pub async fn get_tasks(
    filter: FilterTasks,
    user: JWT,
    conn: Connection<'_, Db>,
) -> Response<Option<GetAllTasks>> {
    // Construct pagination payload from query parameters
    let payload = PaginationPayload {
        page: filter.page.unwrap_or(1) as u64,
        size: filter.size.unwrap_or(10) as u64,
        query: filter.query.clone(),
        user_id: user.claims.sub,
    };

    // Extract database connection
    let db = conn.into_inner();
    
    // Attempt to fetch tasks using provided filters
    let tasks = TaskQueries::get_tasks(payload, db).await;

    match tasks {
        // Return a successful response with the list of tasks
        Ok(tasks_result) => Custom(
            Status::Ok,
            Json(ResponseRequest {
                message: None,
                status: Status::Ok,
                data: Some(tasks_result),
            }),
        ),
        // Return an internal server error response with the error message
        Err(_) => Custom(
            Status::InternalServerError,
            Json(ResponseRequest {
                message: Some("Failed to fetch tasks".to_string()),
                status: Status::InternalServerError,
                data: None,
            }),
        ),
    }
}

/// Endpoint for fetching a single task by ID.
///
/// This function handles the HTTP GET request to fetch a task identified by its `id`.
///
/// # Arguments
///
/// * `id` - The ID of the task to fetch.
/// * `user` - JWT token representing the authenticated user.
/// * `conn` - SeaORM database connection (`Connection<'_, Db>`).
///
/// # Returns
///
/// A custom response (`Response<Option<TaskModel>>`) with status `200 OK` on success,
/// `404 Not Found` if the task is not found, or `500 Internal Server Error` on failure.
///
#[get("/<id>")]
pub async fn get_task(
    id: i32,
    user: JWT,
    conn: Connection<'_, Db>,
) -> Response<Option<TaskModel>> {
    // Extract database connection
    let db = conn.into_inner();
    
    // Attempt to fetch a task by ID using the provided user ID
    let result = TaskQueries::get_task_by_id(id, user.claims.sub, db).await;

    match result {
        // Return a successful response with the task details
        Ok(task) => Custom(
            Status::Ok,
            Json(ResponseRequest {
                message: None,
                status: Status::Ok,
                data: Some(task),
            }),
        ),
        // Return a not found response if the task is not found
        Err(e) => Custom(
            Status::NotFound,
            Json(ResponseRequest {
                message: Some(e.to_string()),
                status: Status::NotFound,
                data: None,
            }),
        ),
    }
}
