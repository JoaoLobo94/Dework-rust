#[macro_use]
mod user_routes {
extern crate rocket;
use rocket::response::status;
use serde_json::{json, Value};
    #[get("/users")]
    fn get_users() -> Value {
        json!({"id":1, "gamba":5, "menina": "muito aint"})
    }
    #[get("/users/<id>")]
    fn view_user(id: i32) -> Value {
        json!({"id":id, "gamba":5, "menina": "muito aint"})
    }
    #[post("/users", format = "json")]
    fn create_user() -> Value {
        json!({"id":1, "gamba":5, "menina": "muito aint"})
    }
    #[put("/users/<id>", format = "json")]
    fn update_user(id: i32) -> Value {
        json!({"id":id, "gamba":5, "menina": "muito aint"})
    }

    #[delete("/users/<_id>")]
    fn delete_user(_id: i32) -> status::NoContent {
        status::NoContent
    }

    #[catch(404)]

    fn not_found() -> Value {
        json!("not found")
    }
}
