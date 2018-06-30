extern crate core;
extern crate rtm;

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
    let task_id = 1;
    let day = 60 * 60 * 24;

    let task = Task {
        id: task_id,
        title: "Buy a cheese".to_string(),
        added: SystemTime::now(),
        due: SystemTime::now().add(Duration::from_secs(day)),
        list: inbox_list.to_string(),
        notes: "must be some hard cheese".to_string(),
        completed: false,
        priority: "High".to_string()
    };
    let res = tasks.create(&task);
    res.expect(&format!("failed to create a new task {:?}", &task));

    let list = tasks.get_tasks(inbox_list.to_string(), false, SystemTime::now());
    println!("{:?}", list);

    tasks.complete(task_id, true).unwrap();
    let list = tasks.get_tasks(inbox_list.to_string(), true, SystemTime::now());
    println!("{:?}", list);
}
