use actix_web::{
    get, post, put,
    web::{self, Json},
    HttpResponse,
};
use api::models::models::{CreateInfo, UpdateInfo};
use serde::Deserialize;

use crate::services;

#[derive(Debug, Deserialize)]
pub struct GetInfoParams {
    section: String,
    verbosity: i32,
}

#[get("/info?{section}&{verbosity}")]
async fn get_info(info: web::Query<GetInfoParams>) -> HttpResponse {
    services::info::get_info(info.section.to_string(), info.verbosity)
}

#[post("/info/create")]
async fn create_info(new_info: Json<CreateInfo>) -> HttpResponse {
    services::info::add_info(&new_info)
}

#[put("/info/update")]
async fn update_info(new_info: Json<UpdateInfo>) -> HttpResponse {
    services::info::update_info(&new_info)
}
