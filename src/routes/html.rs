use core::models::Task;
use core::service::TaskService;
use rocket::State;
use routes::rocket_contrib::Template;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket::request::FlashMessage;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<Task>,
    error: Option<String>,
}

#[derive(FromForm)]
struct DeleteTask {
    task_id: i32
}

const ALL_INCOMPLETE: &str = "All incomplete tasks";

#[get("/tasks/incomplete")]
fn all_incomplete_html(service: State<TaskService>, flash: Option<FlashMessage>) -> Template {
    let tasks = service
        .get_tasks(None, false, None)
        .map_err(|e| format!("get tasks error {}", e));
    let (items, error) = match tasks {
        Ok(t) => (t, None),
        Err(e) => (vec![], Some(e)),
    };

    let msg = flash.map(|msg| format!("{}: {}", msg.name(), msg.msg()));

    let context = TemplateContext {
        name: ALL_INCOMPLETE.to_string(),
        items,
        error:  msg.map(|m| m + &error.unwrap_or("".to_string())),
    };
    Template::render("index", &context)
}

#[post("/tasks/delete", data = "<form>")]
fn delete(service: State<TaskService>, form: Form<DeleteTask>) -> Flash<Redirect> {
    let id = form.into_inner().task_id;
    let deleted = service.delete(id);
    const URI: &str = "/tasks/incomplete";

    match deleted {
        Ok(n) => Flash::success(
            Redirect::to(URI), &format!("Removed task {:?}", id)),
        Err(m) => Flash::error(
            Redirect::to(URI), &format!("Failed to remove task {:?}", id)
        )
    }
}
