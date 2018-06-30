use core::models::Task;
use diesel::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use super::schema::tasks::dsl::*;
use std::time::SystemTime;

pub struct TaskService {
    connection: PgConnection
}

impl TaskService {
    pub fn new(conn: PgConnection) -> TaskService {
        TaskService {
            connection: conn
        }
    }

    pub fn create(&self, task: &Task) -> Result<usize, Error> {
        use core::schema::tasks::dsl::*;

        super::diesel::insert_into(tasks)
            .values(task)
            .execute(&self.connection)
    }

    pub fn delete(&self, _id: i32) -> Result<usize, Error> {
        super::diesel::delete(tasks.filter(id.eq(_id)))
            .execute(&self.connection)
    }

    pub fn get_tasks(&self, _list: String, _completed: bool, date: SystemTime) -> Result<Vec<Task>, Error> {
        tasks
            .filter(list.eq(_list))
            .filter(completed.eq(_completed))
            .filter(due.eq(date))
            .limit(25)
            .load::<Task>(&self.connection)
    }

    pub fn complete(&self, _id: i32, done: bool) -> Result<usize, Error> {
        super::diesel::update(tasks.filter(id.eq(_id)))
            .set(completed.eq(done))
            .execute(&self.connection)
    }
}