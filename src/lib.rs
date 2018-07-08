#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate rocket;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2_diesel::ConnectionManager;
use r2d2::{Pool};
use std::env;
use std::time::Duration;

pub mod core;

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok(); // Grabbing ENV vars
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .connection_timeout(Duration::new(5, 0))
        .build(manager)
        .expect("Failed to create pool.")
}