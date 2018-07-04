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
    use rocket::State;
    use rocket_contrib::Json;
    use rtm::core::models::Task;
    use rtm::core::service::TaskService;
    use chrono::Utc;

    #[get("/")]
    fn index() -> &'static str {
        "Hello, RTM!"
    }

    #[get("/tasks/<list>/<completed>")]
    fn tasks_today(service: State<TaskService>, list: String, completed: bool) -> Option<Json<Vec<Task>>> {
        let now = Utc::now().naive_local();
        service.get_tasks(&list, completed, now).ok().map(|l| Json(l))
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
        .mount("/api", routes![index, tasks_today]).launch();
}
