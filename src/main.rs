mod routes;

use database::Db;
use rocket::{routes};
use sea_orm_rocket::Database;
use std::env;

use crate::routes::auth::{me, sign_in, sign_up};
use crate::routes::ping::ping_route;
use crate::routes::task::{create_task, delete_task, get_task, get_tasks, update_task};

/*async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}
*/

/// Asynchronously starts the Rocket API server.
///
/// This function initializes the Rocket framework with configured routes and database connections,
/// then launches the server.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or failure (`Err(rocket::Error)`).
///
#[tokio::main]
async fn start_api() -> Result<(), rocket::Error> {
    // Configure Rocket with database URL from environment variable
    let figment = rocket::Config::figment().merge((
        "databases.sea_orm.url",
        env::var("DATABASE_URL").expect("Database URL not found"),
    ));

    // Custom Rocket instance with configured routes and database attachment
    rocket::custom(figment)
        .mount("/", routes![ping_route])
        .mount(
            "/task",
            routes![create_task, update_task, delete_task, get_tasks, get_task],
        )
        .mount("/auth", routes![sign_in, sign_up, me])
        .attach(Db::init()) // Initialize database connection
        // .attach(AdHoc::try_on_ignite("Migrations", run_migrations)) // Run database migrations
        .launch()           // Launch the Rocket server
        .await              // Await server launch completion
        .map(|_| ())        // Map launch result to Ok(())
}

fn main() {
    // Load environment variables from .env file
    if dotenvy::dotenv().is_err() {
        println!("Error loading .env file!");
    }

    // Start the Rocket API server asynchronously
    let result = start_api();

    // Print message upon server launch completion or failure
    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
