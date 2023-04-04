#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Value};
use rocket::Request;
mod routes;
mod services;

use routes::index::index;
use routes::info::{create_info, get_info, update_info};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index, get_info, create_info, update_info])
}
