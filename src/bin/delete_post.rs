use diesel::prelude::*;
use diesel_example::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::posts;

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let num_deleted = diesel::delete(posts.find(id))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
