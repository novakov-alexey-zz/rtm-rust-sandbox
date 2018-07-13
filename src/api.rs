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
    tasks(&*service, Some(&list), completed, Some(today))
}

#[get("/tasks/yesterday/<list>/<completed>")]
fn list_yesterday(service: State<TaskService>, list: String, completed: bool) -> JsonOrError {
    let yesterday = (Utc::now() - Duration::days(1)).naive_local();
    tasks(&*service, Some(&list), completed, Some(yesterday))
}

#[get("/tasks/incomplete/<list>")]
fn list_incomplete(service: State<TaskService>, list: String) -> JsonOrError {
    tasks(&*service, Some(&list), false, None)
}

#[get("/tasks/incomplete")]
fn all_incomplete(service: State<TaskService>) -> JsonOrError {
    tasks(&*service, None, false, None)
}

fn tasks(service: &TaskService, list: Option<&str>, completed: bool, due: Option<NaiveDateTime>) -> JsonOrError {
    service.get_tasks(list, completed, due).map(|l| Json(l))
}

#[post("/tasks/<list>")]
fn list_create(service: State<TaskService>, list: String) -> Result<Json<usize>, String> {
    let added = Utc::now().naive_local();
    let task = Task {
        id: 0, //TODO: how to auto-generate the Id?
        title,
        added,
        due,
        list,
        notes,
        completed: false,
        priority
    };
    service.create(&task).map(|i| {
        let msg = if i > 0 {
            "inserted".to_string()
        } else {
            format!("failed to insert {}", i)
        };
        Json(msg)
    })
}
