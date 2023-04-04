use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::information;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Info {
    pub id: i32,
    pub section: String,
    pub body: String,
    pub verbosity: i32,
}

#[derive(Insertable, Serialize, AsChangeset)]
#[diesel(table_name = information)]
pub struct NewInfo<'a> {
    pub section: &'a String,
    pub body: &'a String,
    pub verbosity: &'a i32,
}

#[derive(Deserialize)]
pub struct UpdateInfo {
    pub section: Option<String>,
    pub body: Option<String>,
    pub verbosity: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateInfo {
    pub section: String,
    pub body: String,
    pub verbosity: i32,
}
