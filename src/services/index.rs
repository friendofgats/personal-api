use actix_web::{http::header::ContentType, HttpResponse};

pub fn home() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Hdr", "sample"))
        .json("Welcome to my API")
}
