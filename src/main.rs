#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate core;
extern crate rocket;
extern crate rocket_contrib;
extern crate rtm;

use api::*;
use rtm::core::service::TaskService;
use rtm::create_db_pool;

mod api {
    use rocket::State;
    use rocket_contrib::Json;
    use rtm::core::models::Task;
    use rtm::core::service::TaskService;
    use chrono::{NaiveDateTime, Duration, Utc};

    #[get("/")]
    fn index() -> &'static str {
        "Hello, RTM!"
    }

    #[get("/tasks/today/<list>/<completed>")]
    fn list_today(service: State<TaskService>, list: String, completed: bool) -> Option<Json<Vec<Task>>> {
        let today = Utc::now().naive_local();
        tasks(&*service, &list, completed, today)
    }

    #[get("/tasks/yesterday/<list>/<completed>")]
    fn list_yesterday(service: State<TaskService>, list: String, completed: bool) -> Option<Json<Vec<Task>>> {
        let yesterday = (Utc::now() - Duration::days(1)).naive_local();
        tasks(&*service, &list, completed, yesterday)
    }

    fn tasks(service: &TaskService, list: &str, completed: bool, due: NaiveDateTime) -> Option<Json<Vec<Task>>> {
        service.get_tasks(list, completed, due).ok().map(|l| Json(l))
    }
}

fn service() -> TaskService {
    TaskService::new(create_db_pool())
}

fn main() {
    rocket::ignite()
        .manage(service())
        .mount("/api", routes![index, list_today, list_yesterday]).launch();
}
