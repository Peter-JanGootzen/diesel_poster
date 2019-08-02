#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

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
