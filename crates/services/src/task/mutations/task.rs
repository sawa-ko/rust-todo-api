use database::entities::task::{ActiveModel, Entity, Model};
use sea_orm::*;

/// Struct for handling mutations (create, update, delete) on tasks.
pub struct TaskMutation;

/// Payload structure for creating or updating a task.
pub struct TaskPayload {
    /// The name of the task.
    pub name: String,
    /// The description of the task.
    pub description: String,
    /// Indicates whether the task is active or not.
    pub is_active: bool,
    /// The ID of the user associated with the task.
    pub user_id: i32,
}

impl TaskMutation {
    /// Asynchronously creates a new task with the provided payload.
    ///
    /// # Arguments
    ///
    /// * `task_payload` - Payload containing task details to be created.
    /// * `db` - SeaORM database connection (`&DbConn`).
    ///
    /// # Returns
    ///
    /// A `Result` containing the created `Model` on success, or a `DbErr` on failure.
    pub async fn create(task_payload: TaskPayload, db: &DbConn) -> Result<Model, DbErr> {
        // Create an ActiveModel instance with task payload data
        let active_model = ActiveModel {
            name: Set(task_payload.name.to_owned()),
            description: Set(task_payload.description.to_owned()),
            is_active: Set(task_payload.is_active.to_owned()),
            user_id: Set(task_payload.user_id),
            ..Default::default() // Use default values for unspecified fields
        };

        // Execute the insert operation and await the result
        let res = Entity::insert(active_model).exec(db).await?;

        // Construct and return a Model instance with created task details
        Ok(Model {
            id: res.last_insert_id,
            name: task_payload.name,
            description: task_payload.description,
            is_active: task_payload.is_active,
            user_id: task_payload.user_id,
        })
    }

    /// Asynchronously updates an existing task identified by `id` with the provided payload.
    ///
    /// # Arguments
    ///
    /// * `task_payload` - Payload containing task details to be updated.
    /// * `id` - The ID of the task to be updated.
    /// * `db` - SeaORM database connection (`&DbConn`).
    ///
    /// # Returns
    ///
    /// A `Result` containing the updated `Model` on success, or a `DbErr` on failure.
    pub async fn update(task_payload: TaskPayload, id: i32, db: &DbConn) -> Result<Model, DbErr> {
        // Fetch the task by ID and convert it into an ActiveModel
        let mut task: ActiveModel = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(String::from("Task not found.")))
            .map(Into::into)?; // Convert found entity into ActiveModel

        // Check if the user ID in the task matches the user ID in the payload
        if task.user_id.clone().unwrap() != task_payload.user_id {
            return Err(DbErr::RecordNotFound(String::from("Task not found.")));
        }

        // Update task fields with new values from the payload
        task.name = Set(task_payload.name.to_owned());
        task.description = Set(task_payload.description.to_owned());
        task.is_active = Set(task_payload.is_active.to_owned());

        // Execute the update operation and await the result
        task.update(db).await
    }

    /// Asynchronously deletes an existing task identified by `id` and `user_id`.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the task to be deleted.
    /// * `user_id` - The ID of the user deleting the task.
    /// * `db` - SeaORM database connection (`&DbConn`).
    ///
    /// # Returns
    ///
    /// A `Result` containing the deletion result on success, or a `DbErr` on failure.
    pub async fn delete(id: i32, user_id: i32, db: &DbConn) -> Result<DeleteResult, DbErr> {
        // Fetch the task by ID and convert it into an ActiveModel
        let task: ActiveModel = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(String::from("Task not found.")))
            .map(Into::into)?;

        // Check if the user ID in the task matches the provided user ID
        if task.user_id.clone().unwrap() != user_id {
            return Err(DbErr::RecordNotFound(String::from("Task not found.")));
        }

        // Execute the delete operation and await the result
        task.delete(db).await
    }

    /// Asynchronously deletes all tasks from the database.
    ///
    /// # Arguments
    ///
    /// * `db` - SeaORM database connection (`&DbConn`).
    ///
    /// # Returns
    ///
    /// A `Result` containing the deletion result on success, or a `DbErr` on failure.
    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        // Execute the delete operation for all tasks and await the result
        Entity::delete_many().exec(db).await
    }
}
