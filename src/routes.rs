extern crate rocket;
extern crate rocket_contrib;

use chrono::{Duration, NaiveDateTime, Utc};
use core::models::NewTask;
use core::models::Task;
use core::service::TaskService;
use rocket::Rocket;
use rocket::State;
use routes::rocket_contrib::Json;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
type VecOrError = Result<Json<Vec<Task>>, String>;

#[derive(Deserialize)]
pub struct NewTaskReq {
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
fn list_today(service: State<TaskService>, list: String, completed: bool) -> VecOrError {
    let today = Utc::now().naive_local();
    tasks(&*service, Some(&list), completed, Some(today))
}

#[get("/tasks/yesterday/<list>/<completed>")]
fn list_yesterday(service: State<TaskService>, list: String, completed: bool) -> VecOrError {
    let yesterday = (Utc::now() - Duration::days(1)).naive_local();
    tasks(&*service, Some(&list), completed, Some(yesterday))
}

#[get("/tasks/incomplete/<list>")]
fn list_incompleted(service: State<TaskService>, list: String) -> VecOrError {
    tasks(&*service, Some(&list), false, None)
}

#[get("/tasks/incomplete")]
fn all_incompleted(service: State<TaskService>) -> VecOrError {
    tasks(&*service, None, false, None)
}

fn tasks(
    service: &TaskService,
    list: Option<&str>,
    completed: bool,
    due: Option<NaiveDateTime>,
) -> VecOrError {
    service.get_tasks(list, completed, due).map(|l| Json(l))
}

#[post("/tasks", format = "application/json", data = "<new_task>")]
fn create(service: State<TaskService>, new_task: Json<NewTaskReq>) -> Result<Json<String>, String> {
    let added = Utc::now().naive_local();
    let t = &*new_task;
    let due = NaiveDateTime::parse_from_str(&t.due, DATE_FORMAT);

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
                    Ok(Json(format!("rows inserted {}", i)))
                } else {
                    Err(format!("failed to insert a row, returned number: {}", i))
                }
            })
        }
        Err(pe) => Err(format!("Failed to parse due date: {}", pe)),
    }
}

#[put("/tasks/<task_id>/<complete>")]
fn complete(
    service: State<TaskService>,
    task_id: i32,
    complete: bool,
) -> Result<Json<String>, String> {
    service.complete(task_id, complete).and_then(|i| {
        if i > 0 {
            Ok(Json(format!("rows updated {}", i)))
        } else {
            Err(format!("Failed to update a task, returned number: {}", i))
        }
    })
}

pub fn mount_routes(service: TaskService) -> Rocket {
    rocket::ignite().manage(service).mount(
        "/api",
        routes![
            index,
            list_today,
            list_yesterday,
            list_incompleted,
            all_incompleted,
            create,
            complete
        ],
    )
}
