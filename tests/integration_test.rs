extern crate core;
extern crate rtm;
extern crate chrono;

use rtm::core::service::TaskService;
use rtm::establish_connection;
use rtm::core::models::Task;
use chrono::NaiveDate;

#[test]
fn it_update_task() {
    //given
    let connection = establish_connection();
    let tasks = TaskService::new(connection);
    let inbox_list = "Inbox";
    let task_id = 1;
    let added = NaiveDate::from_ymd(2018, 7, 8).and_hms(9, 10, 11);
    let due = NaiveDate::from_ymd(2018, 7, 9).and_hms(9, 10, 11);

    let task = Task {
        id: task_id,
        title: "Buy a cheese".to_string(),
        added,
        due,
        list: inbox_list.to_string(),
        notes: "must be some hard cheese".to_string(),
        completed: false,
        priority: "High".to_string()
    };

    //when
    let deleted_rows = tasks.delete(task_id).unwrap();
    println!("rows deleted: {:?}", deleted_rows);

    let res = tasks.create(&task);
    let inserted_rows = res.expect("failed to create a new task");
    //then
    assert_eq!(1, inserted_rows);

    //when
    let list = tasks.get_tasks(inbox_list.to_string(), false, due).unwrap();
    //then
    assert_eq!(1, list.len());

    //when
    tasks.complete(task_id, true).unwrap();
    let list = tasks.get_tasks(inbox_list.to_string(), true, due).unwrap();
    //then
    assert_eq!(1, list[0].id);
    assert_eq!(true, list[0].completed);
}