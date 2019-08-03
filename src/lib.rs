#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use self::models::{Post, NewPost};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("You must set the DATABASE_URL in .env");
    PgConnection::establish(&database_url)
        .expect(&format!("Could not connect to the PostgreSQL server at {}", database_url))
}

pub fn create_post<'a>(connection: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(connection)
        .expect("Error saving the post")
}
