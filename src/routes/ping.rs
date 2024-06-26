use memory_stats::memory_stats;
use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use sea_orm::EntityTrait;
use sea_orm_rocket::Connection;
use database::entities::task::Entity as TaskEntity;
use database::Db;

use crate::routes::ResponseRequest;

#[derive(Serialize, Deserialize)]
pub struct PingApi {
    db_status: bool,
    tasks_total: usize,
    memory_usage_mb: String,
}

#[get("/ping")]
pub async fn ping_route(conn: Connection<'_, Db>) -> Json<ResponseRequest<PingApi>> {
    let db = conn.into_inner();
    let db_ping = db.ping().await;
    let total_tasks = TaskEntity::find().all(db).await;

    let result = ResponseRequest {
        message: None,
        status: 200,
        data: PingApi {
            db_status: db_ping.is_ok(),
            tasks_total: total_tasks.unwrap().len(),
            memory_usage_mb: format!("{} Mb", memory_stats().unwrap().physical_mem / 1024 / 1024),
        },
    };

    Json(result)
}
