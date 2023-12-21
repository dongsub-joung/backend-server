use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use commentServer::*;
use diesel::{prelude::*, result::Error};
use serde_json;
use uuid::Uuid;

pub mod service {
    use commentServer::models::{Post, UpdatePost};
    use diesel::associations::HasTable;

    use super::*;

    pub fn show_post() -> Vec<Post> {
        use self::schema::comm::dsl::*;

        let connection = &mut establish_connection();
        let results = comm
            .limit(10)
            .select(Post::as_select())
            .load(connection)
            .expect("Error loading posts");

        results
    }

    pub fn create_post(conn: &mut PgConnection, in_bodying: &str) -> Result<String, Error> {
        use self::schema::comm::dsl::*;
        use commentServer::models::{NewPost, Post};
        // use uuid::Uuid;

        let uuid_v4 = Uuid::new_v4();
        let uuid_v4_str = format!("{:?}", uuid_v4);
        let uuid_v4_str = uuid_v4_str.as_str();

        let new_post = NewPost {
            uuid: uuid_v4_str,
            in_body: in_bodying,
        };

        diesel::insert_into(comm::table())
            .values(&new_post)
            .returning(Post::as_returning())
            .get_result(conn)
            .expect("Insert Err");

        Ok("Add_Success".to_string())
    }

    pub fn get_post(post_id: i32) -> Result<Option<Post>, Error> {
        use self::schema::comm::dsl::*;

        let connection = &mut establish_connection();

        let a_post = comm
            .find(post_id)
            .select(Post::as_select())
            .first(connection)
            .optional(); // This allows for returning an Option<Post>, otherwise it will throw an error

        a_post
    }

    pub fn update_comment(id: i32, in_body: String) -> Post {
        use self::schema::comm::dsl::comm;

        let connection = &mut establish_connection();
        let in_body = in_body.as_str();
        let updated_post = models::UpdatePost { in_body };

        let post = diesel::update(comm.find(id))
            .set(updated_post)
            .returning(Post::as_returning())
            .get_result(connection)
            .unwrap();

        post
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

use actix_cors::Cors;
use actix_web::http::header;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
    .allowed_origin("https://myapp-api.ngrok.dev/comment/all'")
    .allowed_methods(vec!["GET", "POST"])
    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
    .allowed_header(header::CONTENT_TYPE)
    .max_age(3600);
        App::new()
            .wrap(cors)
            .service(hello)
            .service(comment_add)
            .service(comment_list_up)
            .service(comment_getter)
            .service(comment_update)
    })
    .bind(("127.0.0.1", 7080))?
    .run()
    .await
}
