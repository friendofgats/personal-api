use actix_cors::Cors;
use actix_web::{http, App, HttpServer};
use dotenvy::dotenv;
use std::env;

mod routes;
mod services;

use routes::info::{create_info, get_info, update_info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let portfolio_ui_url = env::var("PORTFOLIO_UI_URL").expect("PORTFOLIO_UI_URL must be set");
    let production_flag = env::var("PRODUCTION_FLAG").unwrap_or("0".to_string());
    let ip = if production_flag.eq("1") {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    }
    .to_string();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&portfolio_ui_url)
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(create_info)
            .service(get_info)
            .service(update_info)
    })
    .bind((ip, 8000))?
    .run()
    .await
}
