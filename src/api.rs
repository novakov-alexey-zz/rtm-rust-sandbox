extern crate rocket;
extern crate rocket_contrib;

use chrono::{Duration, NaiveDateTime, Utc};
use rocket::request::Form;
use rocket::State;
use rocket_contrib::Json;
use rtm::core::models::{NewTask, Task};
use rtm::core::service::TaskService;

type JsonOrError = Result<Json<Vec<Task>>, String>;

#[derive(FromForm)]
struct NewTaskForm {
    pub title: String,
    pub due: String,
    pub list: String,
    pub notes: String,
    pub priority: String,
}

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

#[post("/tasks", data = "<form>")]
fn list_create(service: State<TaskService>, form: Form<NewTaskForm>) -> Result<Json<usize>, String> {
    let added = Utc::now().naive_local();
    let t = form.get();
    let due = NaiveDateTime::parse_from_str(&t.due, "%Y-%m-%d %H:%M:%S");

    match due {
        Ok(d) => {
            let task = NewTask {
                title: t.title.clone(),
                added,
                due: d,
                list: t.list.clone(),
                notes: t.notes.clone(),
                priority: t.priority.clone(),
            };

            service.create_new(&task).and_then(|i| {
                if i > 0 {
                    Ok(Json(i))
                } else {
                    Err(format!("failed to insert a row: {}", i))
                }
            })
        }
        Err(pe) => Err(pe.to_string())
    }
}
