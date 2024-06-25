use sea_orm::*;
use database::entities::task::{Entity, Model, Column};

pub struct Queries;

pub struct PaginationPayload {
    pub page: u64,
    pub size: u64,
    pub query: Option<String>,
}

impl Queries {
    pub async fn get_task_by_id(id: i32, db: &DbConn) -> Result<Model, DbErr> {
        Entity::find_by_id(id).one(db).await?.ok_or(DbErr::Custom("Task not found".to_string()))
    }
    
    pub async fn get_tasks(pagination_payload: PaginationPayload, db: &DbConn) -> Result<(Vec<Model>, u64), DbErr> {
        let query = pagination_payload.query.unwrap_or("".to_string());
        let page = pagination_payload.page;
        let limit = pagination_payload.size;
        
        let paginator = Entity::find()
            .filter(Column::Name.contains(query))
            .paginate(db, limit);
        
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|t| (t, num_pages))
    }
}