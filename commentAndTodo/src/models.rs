use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::todoss;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todoss)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub user_id: String,
    pub todotext: String,
}

#[derive(Insertable)]
#[diesel(table_name = todoss)]
pub struct NewTodo<'a> {
    pub user_id: &'a str,
    pub todotext: &'a str,
}


use crate::schema::commss;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::commss)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub user_id: String,
    pub in_body: String,
}

#[derive(Insertable)]
#[diesel(table_name = commss)]
pub struct NewPost<'a> {
    pub user_id: &'a str,
    pub in_body: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = commss)]
#[derive(AsChangeset)]
pub struct UpdatePost<'a> {
    pub in_body: &'a str,
}