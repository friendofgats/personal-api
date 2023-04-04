// extern crate diesel;
// extern crate rocket;
// use crate::models::{self, Info};
// use crate::response::{Content, GenericResponse, SectionResponse};
// use crate::schema;
// use crate::schema::information::section;
// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use dotenvy::dotenv;
// use rocket::get;
// use rocket::http::Status;
// use rocket::response::status;
// use rocket::serde::json::{self, Json};
// use std::env;

// #[get("/healthchecker")]
// pub async fn health_checker_handler() -> Result<Json<GenericResponse>, Status> {
//     const MESSAGE: &str = "Build Simple CRUD API with Rust and Rocket";

//     let response_json = GenericResponse {
//         status: "success".to_string(),
//         message: MESSAGE.to_string(),
//     };
//     Ok(Json(response_json))
// }

// #[get("/<section_filter>")]
// pub fn section_response_handler(section_filter: &str) -> Result<Json<SectionResponse>, Status> {

//     // match ["about"].contains(&section_filter) {
//     //     let err_response = GenericResponse {
//     //         status: "failure".to_string(),
//     //         message: format!("A value for section_filter must be specified. [{}] was the passed value for section_filter.", section_filter).to_string(),
//     //     };
//     //     Err(err_response) => return Err(err_response)
//     // }
//     println!("{}", section_filter);
//     let connection = &mut establish_connection_pg();
//     let results = self::schema::information::dsl::information
//         .filter(section.eq(section_filter))
//         .load::<Info>(connection)
//         .map
//         .expect("Error loading content");

//     match results.eq[] {
//         Ok(results) => {
//             let json_response = SectionResponse {
//                 status: "success".to_string(),
//                 content: Content { content: results },
//             };
//             Ok(Json(json_response));
//         }
//     }
// }

// #[post("/create", format ="application/json", data = "<new_info>")]
// pub fn section_create_handler(new_info: Json<Info>) ->  Result<status::Created<Json<Info>>, Status> {
//     let connection = &mut establish_connection_pg();

//     println!("here 0 {}",&new_info.section);
//     diesel::insert_into(self::schema::information::table)
//         .values(&new_info)
//         .get_result(connection)
// }
