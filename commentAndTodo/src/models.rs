use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::todoss;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todoss)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub todotext: String,
}

#[derive(Insertable)]
#[diesel(table_name = todoss)]
pub struct NewTodo<'a> {
    pub todotext: &'a str,
}


use crate::schema::commss;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::commss)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub uuid: String,
    pub in_body: String,
}

#[derive(Insertable)]
#[diesel(table_name = commss)]
pub struct NewPost<'a> {
    pub uuid: &'a str,
    pub in_body: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = commss)]
#[derive(AsChangeset)]
pub struct UpdatePost<'a> {
    pub in_body: &'a str,
}