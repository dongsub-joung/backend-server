use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn show_post() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .limit(10)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");
    
    // return results
}

use self::models::{NewPost, Post};

pub fn create_post(conn: &mut PgConnection, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")

    // return code
}

// use diesel::prelude::*;
// use diesel_demo::*;

fn get_post(post_id: i32) {
    use self::schema::posts::dsl::posts;

    let connection = &mut establish_connection();

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Post>, otherwise it will throw an error

    // return post
}

fn delete_post(target: &str) {
    use self::schema::posts::dsl::*;
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(body.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    let code= format!("Deleted {} posts", num_deleted);

    // return code
}

fn main() {
    
}