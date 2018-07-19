#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
extern crate chrono;
extern crate core;
extern crate rocket;
extern crate rocket_contrib;
extern crate rtm;
#[macro_use]
extern crate serde_derive;

use routes::start_server;
use rtm::core::service::TaskService;
use rtm::create_db_pool;

mod routes;

fn main() {
    let error = start_server(TaskService::new(create_db_pool()));
    drop(error);
}
