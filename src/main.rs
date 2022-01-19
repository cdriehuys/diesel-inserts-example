use std::env;

use diesel::{PgConnection, Connection};

#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

fn create_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("The 'DATABASE_URL' environment variable is not set.");

    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Failed to connect to database: {}", database_url))
}

fn main() {
    println!("Hello, world!");
}
