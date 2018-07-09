extern crate chrono;
extern crate core;
extern crate rtm;

use chrono::NaiveDate;
use rtm::core::models::Task;
use rtm::core::service::TaskService;
use rtm::create_db_pool;

#[test]
fn it_update_task() {
    //given
    let tasks = TaskService::new(create_db_pool());
    let inbox = "inbox";
    let task_id = 1;
    let added = NaiveDate::from_ymd(2018, 7, 8).and_hms(9, 10, 11);
    let due = NaiveDate::from_ymd(2018, 7, 9).and_hms(9, 10, 11);

    let task = Task {
        id: task_id,
        title: "Buy a cheese".to_string(),
        added,
        due,
        list: inbox.to_string(),
        notes: "must be some hard cheese".to_string(),
        completed: false,
        priority: "High".to_string(),
    };

    //when
    let deleted_rows = tasks.delete(task_id);
    assert!(deleted_rows.is_ok(), "failed to delete a task");
    println!("rows deleted: {:?}", deleted_rows.unwrap());

    let res = tasks.create(&task);
    let inserted_rows = res.expect("failed to create a new task");
    //then
    assert_eq!(1, inserted_rows);

    //when
    let list = tasks.get_tasks(inbox, false, Some(due));
    //then
    assert!(list.is_ok());
    assert_eq!(1, list.unwrap().len());

    //when
    let res = tasks.complete(task_id, true);
    //then
    assert!(res.is_ok());
    //when
    let res = tasks.get_tasks(inbox, true, Some(due));
    //then
    assert!(res.is_ok());
    let list = res.unwrap();
    assert_eq!(1, list[0].id);
    assert_eq!(true, list[0].completed);
}