use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use commentAndTodo::*;
use diesel::{prelude::*, result::Error};
use serde_json;
use uuid::Uuid;

pub mod service {
    use commentAndTodo::models::{Post, UpdatePost};
    use diesel::associations::HasTable;

    use super::*;

    pub fn removing(post_id: i32){
        use self::schema::commss::dsl::*;

        let connection = &mut establish_connection();

        let num_deleted = diesel::delete(commss.filter(id.eq(post_id)))
            .execute(connection)
            .expect("Error deleting posts");
    
    }

    pub fn show_post() -> Vec<Post> {
        use self::schema::commss::dsl::*;

        let connection = &mut establish_connection();
        let results = commss
            .limit(10)
            .select(Post::as_select())
            .load(connection)
            .expect("Error loading posts");

        results
    }

    pub fn create_post(conn: &mut PgConnection, in_bodying: &str) -> Result<String, Error> {
        use self::schema::commss::dsl::*;
        use commentAndTodo::models::{NewPost, Post};

        let uuid_v4 = Uuid::new_v4();
        let uuid_v4_str = format!("{:?}", uuid_v4);
        let uuid_v4_str = uuid_v4_str.as_str();

        let new_post = NewPost {
            uuid: uuid_v4_str,
            in_body: in_bodying,
        };

        diesel::insert_into(commss::table())
            .values(&new_post)
            .returning(Post::as_returning())
            .get_result(conn)
            .expect("Insert Err");

        Ok("Add_Success".to_string())
    }

    pub fn get_post(post_id: i32) -> Result<Option<Post>, Error> {
        use self::schema::commss::dsl::*;

        let connection = &mut establish_connection();

        let a_post = commss
            .find(post_id)
            .select(Post::as_select())
            .first(connection)
            .optional(); // This allows for returning an Option<Post>, otherwise it will throw an error

        a_post
    }

    pub fn update_comment(id: i32, in_body: String) -> Post {
        use self::schema::commss::dsl::commss;

        let connection = &mut establish_connection();
        let in_body = in_body.as_str();
        let updated_post = models::UpdatePost { in_body };

        let post = diesel::update(commss.find(id))
            .set(updated_post)
            .returning(Post::as_returning())
            .get_result(connection)
            .unwrap();

        post
    }
}

pub mod todoService {
    use super::*;
    use todoService::{
        models::{NewTodo, Todo},
        schema::todoss,
    };

    pub fn create_todo(body_text: &str) -> Result<String, Error> {
        let new_todo = NewTodo {
            todotext: body_text,
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

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/comment/add")]
async fn comment_add(req_body: String) -> impl Responder {
    let connection = &mut establish_connection();

    let code = service::create_post(connection, &req_body);

    HttpResponse::Ok().body(code.expect("ERR_ADD"))
}

#[post("/comment/update/{id}")]
async fn comment_update(path: web::Path<(u32,)>, req_body: String) -> impl Responder {
    let connection = &mut establish_connection();
    let ids= path.into_inner().0;
    
    let post_obj = service::update_comment(ids as i32, req_body);
    let json_response =
        serde_json::to_string(&post_obj).expect("Failed to serialize posts to JSON");

    HttpResponse::Ok().body(json_response)
}

#[get("/comment/all")]
async fn comment_list_up() -> impl Responder {
    let posts = service::show_post();

    let json_response = serde_json::to_string(&posts).expect("Failed to serialize posts to JSON");

    // Return the JSON response in the HttpResponse
    HttpResponse::Ok().body(json_response)
}

#[get("/comment/per/{id}")]
async fn comment_getter(path: web::Path<i32>) -> impl Responder {
    let posts = service::get_post(path.into_inner()).unwrap();

    let json_response = serde_json::to_string(&posts).expect("Failed to serialize posts to JSON");

    // Return the JSON response in the HttpResponse
    HttpResponse::Ok().body(json_response)
}

#[get("/comment/remove/{id}")]
async fn comment_remove(path: web::Path<i32>) -> impl Responder {
    let _ = service::removing(path.into_inner());

    // Return the JSON response in the HttpResponse
    HttpResponse::Ok().body("Removed")
}


// TODO

#[post("/todo/add")]
async fn todo_add(body_text: String) -> impl Responder {
    let code = todoService::create_todo(&body_text);

    HttpResponse::Ok().body(code.expect("ERR_ADD"))
}

#[get("/todo/remove/{_id}")]
async fn todo_remove(_id: web::Path<(i32,)>) -> impl Responder {
    let _id= _id.into_inner().0;
    todoService::todo_remove(_id);

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

    todoService::todo_remove(_id);

    HttpResponse::Ok().body("Successed remove")
}


#[get("/todos")]
async fn todo_list_up() -> impl Responder {
    let todo_list = todoService::show_post();

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
