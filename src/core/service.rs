use core::models::Task;
use diesel::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use super::schema::tasks::dsl::*;
use chrono::NaiveDateTime;

pub struct TaskService {
    connection: PgConnection
}

//TODO: what should be in the impl
unsafe impl Send for TaskService {}
unsafe impl Sync for TaskService {}

impl TaskService {
    pub fn new(conn: PgConnection) -> TaskService {
        TaskService {
            connection: conn
        }
    }

    pub fn create(&self, task: &Task) -> Result<usize, Error> {
        super::diesel::insert_into(tasks)
            .values(task)
            .execute(&self.connection)
    }

    pub fn delete(&self, _id: i32) -> Result<usize, Error> {
        super::diesel::delete(tasks.filter(id.eq(_id)))
            .execute(&self.connection)
    }

    pub fn get_tasks(&self, _list: &str, _completed: bool, date: NaiveDateTime) -> Result<Vec<Task>, Error> {
        tasks
            .filter(list.eq(_list.to_string()))
            .filter(completed.eq(_completed))
            .filter(due.ge(date))
            .limit(25)
            .load::<Task>(&self.connection)
    }

    pub fn complete(&self, _id: i32, done: bool) -> Result<usize, Error> {
        super::diesel::update(tasks.filter(id.eq(_id)))
            .set(completed.eq(done))
            .execute(&self.connection)
    }
}