mod routes;

use rocket::{launch, routes};

use crate::routes::ping::ping_route;
use crate::routes::task::{create_task, delete_task, get_tasks, update_task};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ping_route]).mount("/", routes![create_task]).mount("/", routes![update_task]).mount("/", routes![delete_task]).mount("/", routes![get_tasks])
}