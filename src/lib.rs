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
use std::env;
use r2d2_diesel::ConnectionManager;
use r2d2::{Pool, PooledConnection};
use rocket::request::FromRequest;
use rocket::{Request, State};
use rocket::http::Status;
use std::ops::Deref;
use rocket::outcome::Outcome;

pub mod core;

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok(); // Grabbing ENV vars
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create pool.")
}

pub struct DbConn(PooledConnection<ConnectionManager<PgConnection>>);

impl DbConn {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = (); // Associated type, we must have an error we can `Debug`

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), Self::Error> {
        let pool = request.guard::<State<Pool<ConnectionManager<PgConnection>>>>()?;

        // Here were are using the `get()` method from the connection pool to grab
        // the connection. If it's Ok, return the DbConn tuple-struct we made
        // wrapped in an `Outcome` to conform to the function signature.
        // If the `get()` returns an Error, we're returning a tuple with the
        // signature (SomeFailureType, ())
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}