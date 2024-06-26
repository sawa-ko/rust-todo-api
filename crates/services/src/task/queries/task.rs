use rocket::futures::FutureExt;
use database::entities::task::{Column, Entity, Model};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use crate::task::models::task::TaskModel;

pub struct TaskQueries;

pub struct PaginationPayload {
    pub page: u64,
    pub size: u64,
    pub query: Option<String>,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GetAllTasks {
    items: Vec<TaskModel>,
    num_pages: u64,
    size: u64,
    page: u64,
}

impl TaskQueries {
    pub async fn get_task_by_id(id: i32, user_id: i32, db: &DbConn) -> Result<TaskModel, DbErr> {
        let task: TaskModel = Entity::find_by_id(id)
            .find_also_related(database::entities::user::Entity)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Task not found.".to_string()))?
            .into();

        Ok(task)
    }

    pub async fn get_tasks(
        pagination_payload: PaginationPayload,
        db: &DbConn,
    ) -> Result<GetAllTasks, DbErr> {
        let query = pagination_payload.query.unwrap_or("".to_string());
        let page = pagination_payload.page;
        let size = pagination_payload.size;

        let paginator = Entity::find()
            .find_also_related(database::entities::user::Entity)
            .filter(Column::Name.contains(query))
            .filter(Column::UserId.eq(pagination_payload.user_id))
            .paginate(db, pagination_payload.size);

        let num_pages = paginator.num_pages().await?;
        let items: Vec<TaskModel> = paginator.fetch_page(page - 1).await?.into_iter().map(TaskModel::from).collect();

        Ok(GetAllTasks {
            num_pages,
            size,
            page,
            items,
        })
    }
}
