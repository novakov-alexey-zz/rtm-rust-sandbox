extern crate chrono;
extern crate core;
extern crate rtm;

use chrono::Local;
use rtm::core::service::TaskService;
use rtm::establish_connection;

fn main() {
    let connection = establish_connection();
    let tasks = TaskService::new(connection);
    let list = tasks.get_tasks("Inbox".to_string(), false, Local::now());
    println!("{:?}", list);
    tasks.complete(10, true).unwrap();
}
