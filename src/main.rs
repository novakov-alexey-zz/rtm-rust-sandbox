#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate core;
extern crate rocket;
extern crate rocket_contrib;
extern crate rtm;

use api::*;
use rtm::core::service::TaskService;
use rtm::establish_connection;

mod api {
    use chrono::NaiveDate;
    use rocket::State;
    use rocket_contrib::Json;
    use rtm::core::models::Task;
    use rtm::core::service::TaskService;

    #[get("/")]
    fn index() -> &'static str {
        "Hello, RTM!"
    }

    #[get("/tasks/<list>/<completed>")]
    fn tasks(service: State<TaskService>, list: String, completed: bool) -> Option<Json<Vec<Task>>> {
        let today = NaiveDate::from_ymd(2018, 7, 1).and_hms(9, 10, 11);
        service.get_tasks(&list, completed, today).ok().map(|l| Json(l))
    }
}

fn service() -> TaskService {
    let connection = establish_connection();
    let tasks = TaskService::new(connection);
    tasks
}

fn main() {
    rocket::ignite()
        .manage(service())
        .mount("/api", routes![index, tasks]).launch();
}
