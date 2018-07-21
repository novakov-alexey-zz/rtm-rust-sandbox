use routes::rocket_contrib::Template;
use core::service::TaskService;
use core::models::Task;
use rocket::State;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<Task>,
}

#[get("/tasks/incomplete")]
fn all_incompleted_html(service: State<TaskService>) -> Template {
    let res = service.get_tasks(None, false, None).unwrap();
    let context = TemplateContext {
        name: "All incompleted tasks".to_string(),
        items: res,
    };

    Template::render("index", &context)
}
