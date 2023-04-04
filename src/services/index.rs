use rocket::serde::json::{json, Value};

pub fn home() -> Value {
    json!({"message": String::from("Welcome to my API"), "body": {}})
}
