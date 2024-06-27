use memory_stats::memory_stats;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use sea_orm::EntityTrait;
use sea_orm_rocket::Connection;
use database::entities::task::Entity as TaskEntity;
use database::Db;

use crate::routes::ResponseRequest;

/// Struct representing the response data for the ping API endpoint.
#[derive(Serialize, Deserialize)]
pub struct PingApi {
    /// Indicates the status of the database connection (`true` for successful ping, `false` otherwise).
    db_status: bool,
    /// Total number of tasks retrieved from the database.
    tasks_total: usize,
    /// Current memory usage in megabytes formatted as a string.
    memory_usage: String,
}

/// Handler function for the ping API endpoint.
///
/// This function asynchronously retrieves database status, total tasks count, and memory usage,
/// then constructs and returns a JSON response with `PingApi` data.
///
/// # Arguments
///
/// * `conn` - SeaORM database connection (`Connection<'_, Db>`).
///
/// # Returns
///
/// A JSON response containing `ResponseRequest<PingApi>` with status `200 OK`.
///
#[get("/")]
pub async fn ping_route(conn: Connection<'_, Db>) -> Json<ResponseRequest<PingApi>> {
    let db = conn.into_inner();

    // Check database connection status
    let db_ping = db.ping().await;

    // Retrieve total number of tasks from the database
    let total_tasks = TaskEntity::find().all(db).await;

    // Calculate memory usage in megabytes
    let memory_mb = memory_stats().unwrap().physical_mem / 1024 / 1024;
    let memory_usage = format!("{} Mb", memory_mb);

    // Construct response data
    Json(ResponseRequest {
        message: None,
        status: Status::Ok,
        data: PingApi {
            db_status: db_ping.is_ok(),
            tasks_total: total_tasks.unwrap().len(),
            memory_usage,
        },
    })
}
