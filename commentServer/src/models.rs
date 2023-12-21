use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::comm;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::comm)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub uuid: String,
    pub in_body: String,
}

#[derive(Insertable)]
#[diesel(table_name = comm)]
pub struct NewPost<'a> {
    pub uuid: &'a str,
    pub in_body: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = comm)]
#[derive(AsChangeset)]
pub struct UpdatePost<'a> {
    pub in_body: &'a str,
}