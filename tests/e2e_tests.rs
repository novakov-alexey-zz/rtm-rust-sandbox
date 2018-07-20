#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
extern crate chrono;
extern crate core;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rtm;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;
use rocket_contrib::Json;
use rtm::core::service::TaskService;
use rtm::create_db_pool;
use rtm::routes::mount_routes;

#[test]
fn e2e_create_task() {
    let rocket = mount_routes(TaskService::new(create_db_pool()));
    let client = Client::new(rocket).expect("valid rocket instance");
    let new_task = json!({
	    "title": "test",
	    "due": "2018-07-20 12:12:12",
	    "list": "Inbox",
        "notes": "This is a test task 2",
        "priority": "High"
        });
    let mut response = client
        .post("/api/tasks")
        .body(Json(new_task).to_string())
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("\"rows inserted 1\"".into()));
    println!("new task response {:?}", response);
}
