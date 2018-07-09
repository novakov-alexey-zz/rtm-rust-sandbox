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

    type JsonOrError = Result<Json<Vec<Task>>, String>;

    #[get("/")]
    fn index() -> &'static str {
        "Hello, RTM!"
    }

    #[get("/tasks/today/<list>/<completed>")]
    fn list_today(service: State<TaskService>, list: String, completed: bool) -> JsonOrError {
        let today = Utc::now().naive_local();
        tasks(&*service, &list, completed, today)
    }

    #[get("/tasks/yesterday/<list>/<completed>")]
    fn list_yesterday(service: State<TaskService>, list: String, completed: bool) -> JsonOrError {
        let yesterday = (Utc::now() - Duration::days(1)).naive_local();
        tasks(&*service, &list, completed, yesterday)
    }

    //TODO: change to Result<Json, Error> return type
    fn tasks(service: &TaskService, list: &str, completed: bool, due: NaiveDateTime) -> Result<Json<Vec<Task>>, String> {
        service.get_tasks(list, completed, due).map(|l| Json(l))
    }
}

fn main() {
    rocket::ignite()
        .manage(TaskService::new(create_db_pool()))
        .mount("/api", routes![index, list_today, list_yesterday])
        .launch();
}
