use diesel_example::*;

fn main() {
    let connection = &mut establish_connection();
    let results = load_posts(connection);

    println!("Displaying {} posts", results.len());

    for post in results {
        println!("{}", post.title);
        println!("===\n");
        println!("{}", post.body);
    }
}
