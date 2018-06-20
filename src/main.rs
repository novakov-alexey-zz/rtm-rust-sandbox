extern crate chrono;
extern crate core;
extern crate rtm;

use chrono::Local;
use rtm::core::service::TaskService;
use rtm::establish_connection;
use rtm::core::models::Task;
use std::time::SystemTime;
use std::time::Duration;
use std::ops::Add;

fn main() {
    let connection = establish_connection();
    let tasks = TaskService::new(connection);
    let inbox_list = "Inbox";

    let task = Task {
        id: 1,
        title: "Buy a cheese".to_string(),
        added: SystemTime::now(),
        due: SystemTime::now().add(Duration::from_secs(60 * 60 * 24)),
        list: inbox_list.to_string(),
        notes: "must be hard cheese".to_string(),
        completed: false,
        priority: "High".to_string()
    };
    let res = tasks.create(task);
    res.expect("failed to create a new task");

    let list = tasks.get_tasks(inbox_list.to_string(), false, Local::now());
    println!("{:?}", list);

    tasks.complete(1, true).unwrap();
    let list = tasks.get_tasks(inbox_list.to_string(), true, Local::now());
    println!("{:?}", list);
}
