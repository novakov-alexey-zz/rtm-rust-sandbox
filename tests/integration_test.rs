extern crate chrono;
extern crate core;
extern crate rtm;

use chrono::{NaiveDate, NaiveDateTime};
use rtm::core::models::NewTask;
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
    let inbox = "inbox_1";
    let task_id = 1;
    let due = NaiveDate::from_ymd(2018, 5, 9).and_hms(9, 10, 11);

    let task = new_task(inbox, task_id, due);

    //when
    let deleted_rows = service.delete(task_id);
    assert!(deleted_rows.is_ok(), "failed to delete a task");
    println!("rows deleted: {:?}", deleted_rows.unwrap());

    let res = service.insert(&task);
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

#[test]
fn it_create_then_sort() {
    //given
    let service = TaskService::new(create_db_pool());
    let inbox = "inbox_2";
    let task_id = 2;
    let due = NaiveDate::from_ymd(2018, 8, 9).and_hms(9, 10, 11);
    let task1 = new_task(inbox, task_id, due);

    let private = "private";
    let task_id_2 = 3;
    let due_2 = NaiveDate::from_ymd(2018, 7, 9).and_hms(9, 10, 11);
    let task2 = new_task(private, task_id_2, due_2);
    //when
    service.delete(task_id).unwrap();
    service.delete(task_id_2).unwrap();

    let res = service.insert(&task1);
    let inserted_rows = res.expect("failed to create a new task1");
    //then
    assert_eq!(1, inserted_rows);

    //when
    let res = service.insert(&task2);
    let inserted_rows = res.expect("failed to create a new task2");
    //then
    assert_eq!(1, inserted_rows);

    //when
    let today = Some(NaiveDate::from_ymd(2018, 6, 9).and_hms(9, 10, 11));
    let res = service.get_sorted_tasks(None, false, today, TaskSort::DueDate);

    //then
    assert!(res.is_ok());
    let list = res.unwrap();

    assert_eq!(2, list.len());
    assert_eq!(task_id_2, list[0].id);
    assert_eq!(task_id, list[1].id);
}

#[test]
fn it_create_new() {
    //given
    let service = TaskService::new(create_db_pool());
    let inbox = "inbox_3";
    let added = NaiveDate::from_ymd(2018, 5, 8).and_hms(9, 10, 11);
    let due = NaiveDate::from_ymd(2018, 5, 9).and_hms(9, 10, 11);
    let title = "it_create_new";
    let task1 = NewTask {
        title: title.to_string(),
        added,
        due,
        list: inbox.to_string(),
        notes: "must be some hard cheese".to_string(),
        priority: "High".to_string(),
    };

    //when
    service.delete_by_title(title).unwrap();

    let res = service.create_new(&task1);
    let inserted_rows = res.expect("failed to create a new task1");
    //then
    assert_eq!(1, inserted_rows);

    //when
    let res = service.get_sorted_tasks(Some(inbox), false, Some(due), TaskSort::DueDate);

    //then
    assert!(res.is_ok());
    let list = res.unwrap();

    assert_eq!(1, list.len());
    assert_eq!(title, list[0].title);
}
