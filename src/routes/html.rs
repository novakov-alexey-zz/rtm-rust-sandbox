use core::models::Task;
use core::service::TaskService;
use rocket::State;
use routes::rocket_contrib::Template;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<Task>,
}

#[get("/tasks/incomplete")]
fn all_incompleted_html(service: State<TaskService>) -> Template {
    const NAME: &str = "All incompleted tasks";
    let res = service
        .get_tasks(None, false, None)
        .map_err(|e| println!("get tasks error {}", e))
        .unwrap();
    let context = TemplateContext {
        name: NAME.to_string(),
        items: res,
    };

    Template::render("index", &context)
}
