mod routes;

use database::Db;
use migration::MigratorTrait;
use rocket::fairing::AdHoc;
use rocket::{fairing, routes, Build, Rocket};
use sea_orm_rocket::Database;
use std::env;

use crate::routes::auth::{me, sign_in, sign_up};
use crate::routes::ping::ping_route;
use crate::routes::task::{create_task, delete_task, get_task, get_tasks, update_task};

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[tokio::main]
async fn start_api() -> Result<(), rocket::Error> {
    let figment = rocket::Config::figment().merge((
        "databases.sea_orm.url",
        env::var("DATABASE_URL").expect("Database URL not found"),
    ));

    rocket::custom(figment)
        .mount("/", routes![ping_route])
        .mount(
            "/task",
            routes![create_task, update_task, delete_task, get_tasks, get_task],
        )
        .mount("/auth", routes![sign_in, sign_up, me])
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .launch()
        .await
        .map(|_| ())
}

fn main() {
    dotenvy::dotenv().expect("Error loading .env file!");

    let result = start_api();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
