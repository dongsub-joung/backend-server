use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use commentAndTodo::*;
use diesel::{prelude::*, result::Error};
use serde_json;
use uuid::Uuid;
use commentAndTodo::models::{Post, UpdatePost};
use diesel::associations::HasTable;

pub mod service {
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


    // @todo check res value
    pub fn create_post(conn: &mut PgConnection, res_body: &str) -> Result<String, Error> {
        use self::schema::commss::dsl::*;
        use commentAndTodo::models::{NewPost, Post};

        let mut res= res_body.split(",");
        let res_user_id= res.next().expect("null");
        let res_in_body= res.next().expect("null");

        let new_post = NewPost {
            user_id: res_user_id,
            in_body: res_in_body,
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
