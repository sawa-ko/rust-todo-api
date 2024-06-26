use serde::{Deserialize, Serialize};

use database::entities::task::Model;
use database::entities::user as UserEntity;

#[derive(Serialize, Deserialize)]
pub struct TaskModel {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub user_id: i32,
    pub user: Option<UserEntity::Model>,
}

impl From<(Model, Option<UserEntity::Model>)> for TaskModel {
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
