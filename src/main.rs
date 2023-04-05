use actix_cors::Cors;
use actix_web::{http, App, HttpServer};

mod routes;
mod services;

use routes::info::{create_info, get_info, update_info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
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
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
