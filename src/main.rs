#[macro_use]
extern crate rocket;
use serde_json::{json, Value};

#[get("/")]
fn index() -> Value {
    json!("Hello, world!")
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
            ],
        )
}
