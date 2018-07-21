extern crate rocket;
extern crate rocket_contrib;

mod html;
mod rest;

use self::rocket_contrib::Template;
use core::service::TaskService;
use rocket::Rocket;
use routes::html::*;
use routes::rest::*;

pub fn mount_routes(service: TaskService) -> Rocket {
    rocket::ignite()
        .manage(service)
        .mount(
            "/api",
            routes![
                index,
                list_today,
                list_yesterday,
                list_incompleted,
                all_incompleted,
                create,
                complete,
                all_incompleted_html
            ],
        )
        .attach(Template::fairing())
}
