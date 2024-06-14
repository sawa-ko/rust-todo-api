#[macro_use] extern crate rocket;

mod routes;

use crate::routes::ping::ping_route;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ping_route])
}