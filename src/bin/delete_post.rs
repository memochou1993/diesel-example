use diesel_example::*;
use std::env::args;

fn main() {
    let id = args()
        .nth(1)
        .expect("delete_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let num_deleted = delete_post(connection, &id);

    println!("Deleted {} posts", num_deleted);
}
