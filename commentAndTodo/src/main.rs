use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use commentAndTodo::*;
use diesel::{prelude::*, result::Error};
use serde_json;
use uuid::Uuid;

mod comm;
mod todo;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/comment/add")]
async fn comment_add(req_body: String, ) -> impl Responder {
    let connection = &mut establish_connection();

    let code = comm::service::create_post(connection, &req_body);

    HttpResponse::Ok().body(code.expect("ERR_ADD"))
}

#[post("/comment/update/{id}")]
async fn comment_update(path: web::Path<(u32,)>, req_body: String) -> impl Responder {
    let connection = &mut establish_connection();
    let ids= path.into_inner().0;
    
    let post_obj = comm::service::update_comment(ids as i32, req_body);
    let json_response =
        serde_json::to_string(&post_obj).expect("Failed to serialize posts to JSON");

    HttpResponse::Ok().body(json_response)
}

#[get("/comments")]
async fn comment_list_up() -> impl Responder {
    let posts = comm::service::show_post();

    let json_response = serde_json::to_string(&posts).expect("Failed to serialize posts to JSON");

    // Return the JSON response in the HttpResponse
    HttpResponse::Ok().body(json_response)
}

#[get("/comment/per/{id}")]
async fn comment_getter(path: web::Path<i32>) -> impl Responder {
    let posts = comm::service::get_post(path.into_inner()).unwrap();

    let json_response = serde_json::to_string(&posts).expect("Failed to serialize posts to JSON");

    // Return the JSON response in the HttpResponse
    HttpResponse::Ok().body(json_response)
}

#[get("/comment/remove/{id}")]
async fn comment_remove(path: web::Path<i32>) -> impl Responder {
    let _ = comm::service::removing(path.into_inner());

    // Return the JSON response in the HttpResponse
    HttpResponse::Ok().body("Removed")
}




// TODO
#[post("/todo/add")]
async fn todo_add(body_text: String) -> impl Responder {
    let code = todo::todoService::create_todo(&body_text);

    HttpResponse::Ok().body(code.expect("ERR_ADD"))
}

#[get("/todo/remove/{_id}")]
async fn todo_remove(_id: web::Path<(i32,)>) -> impl Responder {
    let _id= _id.into_inner().0;
    todo::todoService::todo_remove(_id);

    HttpResponse::Ok().body("Successed remove")
}

#[post("/todo/per/remove")]
async fn todo_remove_invi_value(mut _id: String) -> impl Responder {
    // if _id.contains("\r") || _id.contains("\n"){
    //     let first= _id.find("\r\n").expect("find frist err");
    //     // let end= _id.find("\n").expect("find end err");
    //     _id= (&_id[first..]).to_string();
    // }

    let _id: i32= _id.trim().trim_end().parse().unwrap();

    todo::todoService::todo_remove(_id);

    HttpResponse::Ok().body("Successed remove")
}


#[get("/todos")]
async fn todo_list_up() -> impl Responder {
    let todo_list = todo::todoService::show_post();

    let json_response =
        serde_json::to_string(&todo_list).expect("Failed to serialize posts to JSON");

    HttpResponse::Ok().body(json_response)
}

use actix_cors::Cors;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(hello)
            .service(comment_add)
            .service(comment_list_up)
            .service(comment_getter)
            .service(comment_update)
            .service(comment_remove)
            .service(todo_add)
            .service(todo_remove)
            .service(todo_list_up)
            .service(todo_remove_invi_value)
    })
    .bind(("127.0.0.1", 7321))?
    .run()
    .await
}
