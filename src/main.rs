mod routes;

use rocket::{routes};
use sea_orm_rocket::Database;
use database::Db;

use crate::routes::ping::ping_route;
use crate::routes::task::{create_task, delete_task, get_tasks, update_task};

#[tokio::main]
async fn start_api() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", routes![ping_route])
        .mount("/task", routes![create_task, update_task, delete_task, get_tasks])
        .attach(Db::init())
        .launch()
        .await
        .map(|_| ())
}

fn main() {
    let result = start_api();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}