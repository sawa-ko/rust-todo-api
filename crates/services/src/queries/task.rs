use sea_orm::*;
use database::entities::task::{Entity, Model, Column};
use serde::{Deserialize, Serialize};

pub struct TaskQueries;

pub struct PaginationPayload {
    pub page: u64,
    pub size: u64,
    pub query: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetAllTasks {
    items: Vec<Model>,
    num_pages: u64,
    size: u64,
    page: u64,
}

impl TaskQueries {
    pub async fn get_task_by_id(id: i32, db: &DbConn) -> Result<Model, DbErr> {
        Entity::find_by_id(id).one(db).await?.ok_or(DbErr::Custom("Task not found".to_string()))
    }
    
    pub async fn get_tasks(pagination_payload: PaginationPayload, db: &DbConn) -> Result<GetAllTasks, DbErr> {
        let query = pagination_payload.query.unwrap_or("".to_string());
        let page = pagination_payload.page;
        let size = pagination_payload.size;
        
        let paginator = Entity::find()
            .filter(Column::Name.contains(query))
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