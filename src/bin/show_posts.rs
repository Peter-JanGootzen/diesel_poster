extern crate diesel_poster;
extern crate diesel;

use self::diesel_poster::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_poster::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Could not load posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
