#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
extern crate chrono;
extern crate core;
extern crate rocket;
extern crate rocket_contrib;
extern crate rtm;
#[macro_use]
extern crate serde_derive;

use api::*;
use rtm::core::service::TaskService;
use rtm::create_db_pool;

mod api;

fn main() {
    rocket::ignite()
        .manage(TaskService::new(create_db_pool()))
        .mount("/api", routes![index, list_today, list_yesterday, list_incomplete, all_incomplete, list_create])
        .launch();
}
