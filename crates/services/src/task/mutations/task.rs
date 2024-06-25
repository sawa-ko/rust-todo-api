use sea_orm::*;
use database::entities::task::{ActiveModel, Entity, Model};

pub struct TaskMutation;

pub struct TaskPayload {
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub user_id: i32,
}

impl TaskMutation {
    pub async fn create(task_payload: TaskPayload, db: &DbConn) -> Result<Model, DbErr> {
        let active_model = ActiveModel {
            name: Set(task_payload.name.to_owned()),
            description: Set(task_payload.description.to_owned()),
            is_active: Set(task_payload.is_active.to_owned()),
            ..Default::default()
        };

        let res = Entity::insert(active_model).exec(db).await?;

        Ok(Model {
            id: res.last_insert_id,
            name: task_payload.name,
            description: task_payload.description,
            is_active: task_payload.is_active,
            user_id: task_payload.user_id,
        })
    }
    
    pub async fn update(task_payload: TaskPayload, id: i32, db: &DbConn) -> Result<Model, DbErr> {
        let mut task: ActiveModel = Entity::find_by_id(id).one(db).await?.ok_or(DbErr::RecordNotFound(String::from("Task not found."))).map(Into::into)?;
        
        task.name = Set(task_payload.name.to_owned());
        task.description = Set(task_payload.description.to_owned());
        task.is_active = Set(task_payload.is_active.to_owned());
        task.user_id = Set(task_payload.user_id.to_owned());
        
        task.update(db).await
    }
    
    pub async fn delete(id: i32, db: &DbConn) -> Result<DeleteResult, DbErr> {
        let task: ActiveModel = Entity::find_by_id(id).one(db).await?.ok_or(DbErr::RecordNotFound(String::from("Task not found."))).map(Into::into)?;
        task.delete(db).await
    }
    
    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Entity::delete_many().exec(db).await
    }
}