use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::todos;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub todotext: String,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub todotext: &'a str,
}