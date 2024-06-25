use database::entities::task::{Column, Entity, Model, Relation};
use sea_orm::*;
use serde::{Deserialize, Serialize};

pub struct TaskQueries;

pub struct PaginationPayload {
    pub page: u64,
    pub size: u64,
    pub query: Option<String>,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GetAllTasks {
    items: Vec<Model>,
    num_pages: u64,
    size: u64,
    page: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GetTask {
    pub task: Model,
    pub user: Option<database::entities::user::Model>,
}

impl TaskQueries {
    pub async fn get_task_by_id(id: i32, user_id: i32, db: &DbConn) -> Result<GetTask, DbErr> {
        let task = Entity::find_by_id(id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Task not found.".to_string()))?;

        let user = task
            .find_related(database::entities::user::Entity)
            .columns([
                database::entities::user::Column::Id,
                database::entities::user::Column::Username,
            ])
            .one(db)
            .await?;

        Ok(GetTask { task, user })
    }

    pub async fn get_tasks(
        pagination_payload: PaginationPayload,
        db: &DbConn,
    ) -> Result<GetAllTasks, DbErr> {
        let query = pagination_payload.query.unwrap_or("".to_string());
        let page = pagination_payload.page;
        let size = pagination_payload.size;

        let paginator = Entity::find()
            .filter(Column::Name.contains(query))
            .filter(Column::UserId.eq(pagination_payload.user_id))
            .paginate(db, pagination_payload.size);

        let num_pages = paginator.num_pages().await?;

        Ok(GetAllTasks {
            num_pages,
            size,
            page,
            items: paginator.fetch_page(page - 1).await?,
        })
    }
}
