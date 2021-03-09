/// Diesel experiment with multiple joins.
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use std::env;

pub mod models;
pub mod schema;

use schema::*;

// requried for implicit inner_join on clause
joinable!(comments -> posts (post_id));
joinable!(comments -> users (user_id));

pub fn establish_connection() -> SqliteConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
