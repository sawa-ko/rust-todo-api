use rocket::get;

#[get("/ping")]
pub fn ping_route() -> String {
    String::from("Hello, world!")
}
