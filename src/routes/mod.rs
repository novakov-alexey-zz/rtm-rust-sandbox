extern crate rocket;
extern crate rocket_contrib;

use core::service::TaskService;
use rocket::Rocket;
use routes::html::*;
use routes::rest::*;
use self::rocket_contrib::Template;

mod html;
mod rest;

pub fn mount_routes(service: TaskService) -> Rocket {
    rocket::ignite()
        .manage(service)
        .mount(
            "/api",
            routes![
                index,
                list_today,
                list_yesterday,
                list_incomplete,
                all_incomplete,
                create,
                complete
            ],
        )
        .mount("/", routes![all_incomplete_html, delete])
        .attach(Template::fairing())
}
