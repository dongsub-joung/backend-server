use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use commentAndTodo::*;
use diesel::{prelude::*, result::Error};
use serde_json;
use uuid::Uuid;

pub mod todoService {
    use super::*;
    
    use todoService::{
        models::{NewTodo, Todo},
        schema::todoss,
    };

    pub fn create_todo(res_json: &str) -> Result<String, Error> {
        let mut res_json_split= res_json.split(",");
        let res_user_id= res_json_split.next().expect("res user id");
        let res_body_txt= res_json_split.next().expect("body text");
        
        let new_todo = NewTodo {
            user_id: res_user_id,
            todotext: res_body_txt,
        };

        let connection = &mut establish_connection();

        diesel::insert_into(todoss::table)
            .values(&new_todo)
            .returning(Todo::as_returning())
            .get_result(connection)
            .expect("Insert Err");

        Ok("Add_Success".to_string())
    }

    pub fn show_post() -> Vec<Todo> {
        use self::schema::todoss::dsl::*;

        let connection = &mut establish_connection();

        let results = todoss
            .select(Todo::as_select())
            .load(connection)
            .expect("Error loading posts");

        results
    }

    pub fn todo_remove(_id: i32) {
        use self::schema::todoss::dsl::*;

        let connection = &mut establish_connection();

        diesel::delete(todoss.filter(id.eq(_id)))
            .execute(connection)
            .expect("Error deleting posts");
    }
}