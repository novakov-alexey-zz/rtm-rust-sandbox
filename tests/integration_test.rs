extern crate core;
extern crate rtm;

use rtm::core::service::TaskService;
use rtm::establish_connection;
use rtm::core::models::Task;
use std::time::{SystemTime, Duration};
use std::ops::Add;

#[test]
fn it_update_task() {
    let connection = establish_connection();
    let tasks = TaskService::new(connection);
    let inbox_list = "Inbox";
    let task_id = 1;
    let day = 60 * 60 * 24;
    let due = SystemTime::now().add(Duration::from_secs(day));

    let task = Task {
        id: task_id,
        title: "Buy a cheese".to_string(),
        added: SystemTime::now(),
        due: due,
        list: inbox_list.to_string(),
        notes: "must be some hard cheese".to_string(),
        completed: false,
        priority: "High".to_string()
    };

    let deleted_rows = tasks.delete(task_id).unwrap();
    println!("rows deleted: {:?}", deleted_rows);

    let res = tasks.create(&task);
    let inserted_rows = res.expect("failed to create a new task");
    assert_eq!(1, inserted_rows);

    let list = tasks.get_tasks(inbox_list.to_string(), false, due).unwrap();
    assert_eq!(1, list.len());

    tasks.complete(task_id, true).unwrap();
    let list = tasks.get_tasks(inbox_list.to_string(), true, due).unwrap();
    assert_eq!(1, list[0].id);
    assert_eq!(true, list[0].completed);
}