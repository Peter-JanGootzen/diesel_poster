extern crate diesel_poster;
extern crate diesel;

use self::diesel_poster::*;
use self::diesel::prelude::*;
use self::models::Post;
use std::env::args;

fn main() {
    use diesel_poster::schema::posts::dsl::{ posts, published };

    let id = args().nth(1).expect("You must provide a post id")
        .parse::<i32>().expect("id must be a valid 32bit integer");
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}
