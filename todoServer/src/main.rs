use actix_cors::Cors;
use actix_web::{get, post, App, web, HttpResponse, HttpServer, Responder};
use diesel::{prelude::*, result::Error};
use serde_json;

use todoServer::{schema::todos, *};

pub mod service {
    use super::*;
    use todoServer::{
        models::{NewTodo, Todo},
        schema::todos,
    };

    pub fn create_todo(body_text: &str) -> Result<String, Error> {
        let new_todo = NewTodo {
            todotext: body_text,
        };

        let connection = &mut establish_connection();

        diesel::insert_into(todos::table)
            .values(&new_todo)
            .returning(Todo::as_returning())
            .get_result(connection)
            .expect("Insert Err");

        Ok("Add_Success".to_string())
    }

    pub fn show_post() -> Vec<Todo> {
        use self::schema::todos::dsl::*;

        let connection = &mut establish_connection();

        let results = todos
            .select(Todo::as_select())
            .load(connection)
            .expect("Error loading posts");

        results
    }

    pub fn todo_remove(_id: i32) {
        use self::schema::todos::dsl::*;

        let connection = &mut establish_connection();

        diesel::delete(todos.filter(id.eq(_id)))
            .execute(connection)
            .expect("Error deleting posts");
    }
}

#[post("/todo/add")]
async fn todo_add(body_text: String) -> impl Responder {
    let code = service::create_todo(&body_text);

    HttpResponse::Ok().body(code.expect("ERR_ADD"))
}

#[get("/todo/remove/{_id}")]
async fn todo_remove(_id: web::Path<(i32,)>) -> impl Responder {
    let _id= _id.into_inner().0;
    service::todo_remove(_id);

    HttpResponse::Ok().body("Successed remove")
}

#[post("/todo/per/remove")]
async fn todo_remove_invi_value(mut _id: String) -> impl Responder {
    if _id.contains("\r") || _id.contains("\n"){
        let first= _id.find("\r\n").expect("find frist err");
        // let end= _id.find("\n").expect("find end err");
        _id= (&_id[first..]).to_string();
    }

    let _id: i32= _id.trim().trim_end().parse().unwrap();

    service::todo_remove(_id);

    HttpResponse::Ok().body("Successed remove")
}


#[get("/todos")]
async fn todo_list_up() -> impl Responder {
    let todo_list = service::show_post();

    let json_response =
        serde_json::to_string(&todo_list).expect("Failed to serialize posts to JSON");

    HttpResponse::Ok().body(json_response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(todo_add)
            .service(todo_remove)
            .service(todo_list_up)
            .service(todo_remove_invi_value)
    })
    .bind(("127.0.0.1", 7081))?
    .run()
    .await
}
