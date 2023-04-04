use actix_web::{get, HttpResponse};

use crate::services;

#[get("/")]
async fn index() -> HttpResponse {
    services::index::home()
}
