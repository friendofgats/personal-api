use api::{
    models::models::{CreateInfo, Info, NewInfo, UpdateInfo},
    *,
};
use diesel::prelude::*;

use rocket::serde::json::{json, Value};

pub fn get_info(req_section: String, mut req_verbosity: i32) -> Value {
    use api::schema::information::dsl::*;
    if req_verbosity > 10 || req_verbosity < 1 {
        req_verbosity = 10
    }
    let connection = &mut establish_connection_pg();
    let results: Option<Info> = information
        .filter(section.eq(req_section))
        .filter(verbosity.ge(req_verbosity))
        .order(verbosity.asc())
        .first::<Info>(connection)
        .optional()
        .expect("Error loading information");
    json!(results)
}

pub fn add_info(new_info: &CreateInfo) -> Value {
    use api::schema::information;

    let connection = &mut establish_connection_pg();
    //TODO check for existing item, if so, update (do all of this in create_or_update func)
    let info_to_create = NewInfo {
        section: &new_info.section,
        body: &new_info.body,
        verbosity: &new_info.verbosity,
    };

    let created_info: Info = diesel::insert_into(information::table)
        .values(&info_to_create)
        .get_result::<Info>(connection)
        .expect("Error saving new info");

    json!(created_info)
}

pub fn update_info(update_info: &UpdateInfo) -> Value {
    use api::schema::information::dsl::*;

    let connection = &mut establish_connection_pg();
    let existing_info = information
        .filter(section.eq(update_info.section.clone().unwrap()))
        .limit(1)
        .load::<Info>(connection)
        .expect("Cannot fetch section info");

    let info_to_update: NewInfo = NewInfo {
        section: &update_info
            .section
            .clone()
            .unwrap_or(existing_info[0].section.clone()),
        body: &update_info
            .body
            .clone()
            .unwrap_or(existing_info[0].body.clone()),
        verbosity: &update_info
            .verbosity
            .clone()
            .unwrap_or(existing_info[0].verbosity.clone()),
    };

    let updated_info: Info =
        diesel::update(information.filter(section.eq(update_info.section.clone().unwrap())))
            .set(&info_to_update)
            .get_result::<Info>(connection)
            .expect("Error updating info");

    json!(updated_info)
}
