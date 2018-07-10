extern crate chrono;
extern crate core;
extern crate rtm;

use chrono::{NaiveDateTime, NaiveDate};
use rtm::core::models::Task;
use rtm::core::service::{TaskService, TaskSort};
use rtm::create_db_pool;

fn new_task(list: &str, task_id: i32, due: NaiveDateTime) -> Task {
    let added = NaiveDate::from_ymd(2018, 7, 8).and_hms(9, 10, 11);

    Task {
        id: task_id,
        title: "Buy a cheese".to_string(),
        added,
        due,
        list: list.to_string(),
        notes: "must be some hard cheese".to_string(),
        completed: false,
        priority: "High".to_string(),
    }
}

#[test]
fn it_create_then_complete_task() {
    //given
    let service = TaskService::new(create_db_pool());
    let inbox = "inbox";
    let task_id = 1;
    let due = NaiveDate::from_ymd(2018, 9, 9).and_hms(9, 10, 11);

    let task = new_task(inbox, task_id, due);

    //when
    let deleted_rows = service.delete(task_id);
    assert!(deleted_rows.is_ok(), "failed to delete a task");
    println!("rows deleted: {:?}", deleted_rows.unwrap());

    let res = service.create(&task);
    let inserted_rows = res.expect("failed to create a new task");
    //then
    assert_eq!(1, inserted_rows);

    //when
    let list = service.get_tasks(Some(inbox), false, Some(due));
    //then
    assert!(list.is_ok());
    assert_eq!(1, list.unwrap().len());

    //when
    let res = service.complete(task_id, true);
    //then
    assert!(res.is_ok());
    //when
    let res = service.get_tasks(Some(inbox), true, Some(due));
    //then
    assert!(res.is_ok());
    let list = res.unwrap();
    assert_eq!(task_id, list[0].id);
    assert_eq!(true, list[0].completed);
}