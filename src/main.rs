use actix_web::{App, HttpServer};

mod routes;
mod services;

use routes::info::{create_info, get_info, update_info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_info)
            .service(get_info)
            .service(update_info)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
