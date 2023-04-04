use api::models::models::{CreateInfo, UpdateInfo};
use rocket::serde::json::{Json, Value};

use crate::services;

#[get("/info?<section>&<verbosity>")]
pub fn get_info(section: String, verbosity: i32) -> Value {
    services::info::get_info(section, verbosity)
}

#[post("/info/create", format = "json", data = "<new_info>")]
pub fn create_info(new_info: Json<CreateInfo>) -> Value {
    services::info::add_info(&new_info)
}

#[put("/info/update", format = "json", data = "<new_info>")]
pub fn update_info(new_info: Json<UpdateInfo>) -> Value {
    services::info::update_info(&new_info)
}
