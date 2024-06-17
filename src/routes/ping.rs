use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use sea_orm_rocket::Connection;
use crate::pool::Db;
use crate::routes::ResponseRequest;

#[derive(Serialize)]
pub struct PingApi {
    db_status: bool,
}

#[get("/ping")]
pub async fn ping_route(conn: Connection<'_, Db>) -> Json<ResponseRequest<PingApi>> {
    let db = conn.into_inner();
    let db_ping = db.ping().await;
    let result = ResponseRequest {
        message: None,
        status: 200,
        data: PingApi {
            db_status: db_ping.is_ok()
        }
    };

    Json(result)
}
