use self::models::{NewPost, Post};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn load_posts(conn: &mut PgConnection) -> Vec<Post> {
    use self::schema::posts::dsl::{posts, published};

    posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(conn)
        .expect("Error loading posts")
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error creating post")
}

pub fn publish_post(conn: &mut PgConnection, id: &i32) -> Post {
    use self::schema::posts::dsl::{posts, published};

    diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(conn)
        .expect("Error updating post")
}

pub fn delete_post(conn: &mut PgConnection, id: &i32) -> usize {
    use self::schema::posts::dsl::posts;

    diesel::delete(posts.find(id))
        .execute(conn)
        .expect("Error deleting post")
}
