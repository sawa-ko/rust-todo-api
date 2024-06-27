use serde::{Deserialize, Serialize};

use database::entities::task::Model;
use database::entities::user as UserEntity;

/// Struct representing a Task with associated user information.
#[derive(Serialize, Deserialize)]
pub struct TaskModel {
    /// The unique identifier of the task.
    pub id: i32,
    /// The name of the task.
    pub name: String,
    /// The description of the task.
    pub description: String,
    /// Flag indicating whether the task is active or not.
    pub is_active: bool,
    /// The ID of the user associated with the task.
    pub user_id: i32,
    /// Optional user entity associated with the task.
    pub user: Option<UserEntity::Model>,
}

/// Conversion implementation from tuple `(Model, Option<UserEntity::Model>)` to `TaskModel`.
impl From<(Model, Option<UserEntity::Model>)> for TaskModel {
    /// Convert a tuple `(Model, Option<UserEntity::Model>)` into a `TaskModel`.
    ///
    /// # Arguments
    ///
    /// * `value` - Tuple containing the task entity and optional user entity.
    ///
    /// # Returns
    ///
    /// A `TaskModel` instance populated with data from the provided entities.
    ///
    fn from(value: (Model, Option<UserEntity::Model>)) -> Self {
        let (task_entity, user_entity) = value;

        Self {
            id: task_entity.id,
            name: task_entity.name,
            description: task_entity.description,
            is_active: task_entity.is_active,
            user_id: task_entity.user_id,
            user: user_entity,
        }
    }
}
