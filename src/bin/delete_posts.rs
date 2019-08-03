extern crate diesel;
extern crate diesel_poster;

use std::env::args;
use self::diesel_poster::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_poster::schema::posts::dsl::*;

    let post_name = args().nth(1).expect("You must provide a post name to query by");
    let query_pattern = format!("%{}%", post_name);
    let connection = establish_connection();

    let deleted_amount = diesel::delete(posts.filter(title.like(query_pattern)))
        .execute(&connection)
        .expect("Error deleting the posts");
    println!("Deleted {} posts", deleted_amount);
}
