#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::{thread, time};
use std::env;
use std::time::Duration;

pub mod core;
pub mod routes;

const DELAY: Duration = time::Duration::from_secs(3);

pub fn establish_connection() -> Result<PgConnection, String> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    fn try(i: i32, url: &str) -> Result<PgConnection, String> {
        match PgConnection::establish(url) {
            Ok(p) => Ok(p),
            Err(e) =>
                if i == 0 {
                    Err((&format!("Error connecting to {}, error: {}", url, e)).to_string())
                } else {
                    println!("Failed to connect. Trying one more time in \
                    {:?}", DELAY);
                    thread::sleep(DELAY);
                    try(i - 1, url)
                }
        }
    }

    try(3, &database_url)
}

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
