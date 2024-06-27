use crate::task::models::task::TaskModel;
use database::entities::task::{Column, Entity};
use sea_orm::*;
use serde::{Deserialize, Serialize};

/// Struct for handling queries related to tasks.
pub struct TaskQueries;

/// Payload structure for pagination and filtering tasks.
pub struct PaginationPayload {
    /// The page number to fetch.
    pub page: u64,
    /// The size (number of items) per page.
    pub size: u64,
    /// Optional query string for filtering tasks by name.
    pub query: Option<String>,
    /// The ID of the user associated with the tasks.
    pub user_id: i32,
}

/// Structure representing the result of fetching all tasks.
#[derive(Serialize, Deserialize)]
pub struct GetAllTasks {
    /// List of task items fetched.
    pub items: Vec<TaskModel>,
    /// Total number of pages based on pagination settings and query results.
    pub num_pages: u64,
    /// Number of items per page.
    pub size: u64,
    /// Current page number fetched.
    pub page: u64,
}

impl TaskQueries {
    /// Asynchronously fetches a task by its ID and user ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the task to fetch.
    /// * `user_id` - The ID of the user associated with the task.
    /// * `db` - SeaORM database connection (`&DbConn`).
    ///
    /// # Returns
    ///
    /// A `Result` containing the fetched `TaskModel` on success, or a `DbErr` on failure.
    pub async fn get_task_by_id(id: i32, user_id: i32, db: &DbConn) -> Result<TaskModel, DbErr> {
        // Fetch the task by ID and ensure it is associated with the provided user ID
        let task: TaskModel = Entity::find_by_id(id)
            .find_also_related(database::entities::user::Entity)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Task not found.".to_string()))?
            .into();

        Ok(task)
    }

    /// Asynchronously fetches tasks based on pagination and filtering criteria.
    ///
    /// # Arguments
    ///
    /// * `pagination_payload` - Payload containing pagination and filtering parameters.
    /// * `db` - SeaORM database connection (`&DbConn`).
    ///
    /// # Returns
    ///
    /// A `Result` containing `GetAllTasks` with fetched task items, pagination details on success,
    /// or a `DbErr` on failure.
    pub async fn get_tasks(
        pagination_payload: PaginationPayload,
        db: &DbConn,
    ) -> Result<GetAllTasks, DbErr> {
        // Extract pagination and query parameters from the payload
        let query = pagination_payload.query.unwrap_or("".to_string());
        let page = pagination_payload.page;
        let size = pagination_payload.size;

        // Construct the paginator for querying tasks
        let paginator = Entity::find()
            .find_also_related(database::entities::user::Entity)
            .filter(Column::Name.contains(query))
            .filter(Column::UserId.eq(pagination_payload.user_id))
            .paginate(db, pagination_payload.size);

        // Retrieve the total number of pages
        let num_pages = paginator.num_pages().await?;

        // Fetch the tasks for the requested page
        let items: Vec<TaskModel> = paginator
            .fetch_page(page - 1)
            .await?
            .into_iter()
            .map(TaskModel::from)
            .collect();

        // Return the fetched tasks along with pagination details
        Ok(GetAllTasks {
            num_pages,
            size,
            page,
            items,
        })
    }
}
