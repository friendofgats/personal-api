use actix_web::{web, App, HttpServer};

mod routes;
mod services;

use routes::index::index;
use routes::info::{create_info, get_info, update_info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api/")
                .service(create_info)
                .service(get_info)
                .service(update_info)
                .service(index),
        )
    })
    .bind(("localhost", 8000))?
    .run()
    .await
}
