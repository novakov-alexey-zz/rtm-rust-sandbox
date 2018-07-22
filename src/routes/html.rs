use core::models::Task;
use core::service::TaskService;
use rocket::State;
use routes::rocket_contrib::Template;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<Task>,
    error: Option<String>,
}

#[get("/tasks/incomplete")]
fn all_incompleted_html(service: State<TaskService>) -> Template {
    const NAME: &str = "All incompleted tasks";
    let tasks = service
        .get_tasks(None, false, None)
        .map_err(|e| format!("get tasks error {}", e));
    let (items, error) = match tasks {
        Ok(t) => (t, None),
        Err(e) => (vec![], Some(e)),
    };

    let context = TemplateContext {
        name: NAME.to_string(),
        items,
        error,
    };
    Template::render("index", &context)
}
