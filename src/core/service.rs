use core::models::Task;
use diesel::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use super::schema::tasks::dsl::*;
use chrono::NaiveDateTime;
use r2d2_diesel::ConnectionManager;
use r2d2::{Pool};

pub struct TaskService {
    connection_pool: Pool<ConnectionManager<PgConnection>>
}

//TODO: what should be in the impl
unsafe impl Send for TaskService {}
unsafe impl Sync for TaskService {}

impl TaskService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> TaskService {
        TaskService {
            connection_pool: pool
        }
    }

    pub fn create(&self, task: &Task) -> Result<usize, Error> {
        let conn = self.connection_pool.clone().get().unwrap();
        super::diesel::insert_into(tasks)
            .values(task)
            .execute(&*conn)
    }

    pub fn delete(&self, _id: i32) -> Result<usize, Error> {
        let conn = self.connection_pool.get().unwrap();
        super::diesel::delete(tasks.filter(id.eq(_id)))
            .execute(&*conn)
    }

    pub fn get_tasks(&self, _list: &str, _completed: bool, date: NaiveDateTime) -> Result<Vec<Task>, Error> {
        let conn = self.connection_pool.get().unwrap();
        tasks
            .filter(list.eq(_list.to_string()))
            .filter(completed.eq(_completed))
            .filter(due.ge(date))
            .limit(25)
            .load::<Task>(&*conn)
    }

    pub fn complete(&self, _id: i32, done: bool) -> Result<usize, Error> {
        let conn = self.connection_pool.get().unwrap();
        super::diesel::update(tasks.filter(id.eq(_id)))
            .set(completed.eq(done))
            .execute(&*conn)
    }
}