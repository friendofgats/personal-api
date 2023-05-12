use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::information;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Info {
    pub id: i32,
    pub body: String,
    pub section: String,
    pub verbosity: i32,
}

#[derive(Insertable, Serialize, AsChangeset)]
#[diesel(table_name = information)]
pub struct NewInfo<'a> {
    pub body: &'a String,
    pub section: &'a String,
    pub verbosity: &'a i32,
}

#[derive(Deserialize)]
pub struct UpdateInfo {
    pub body: Option<String>,
    pub section: Option<String>,
    pub verbosity: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateInfo {
    pub body: String,
    pub section: String,
    pub verbosity: i32,
}
