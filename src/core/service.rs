use core::models::Task;
use diesel::*;
use diesel::pg::PgConnection;
use super::schema::tasks::dsl::*;
use chrono::NaiveDateTime;
use r2d2_diesel::ConnectionManager;
use r2d2::{Pool, PooledConnection};
use std::error;

pub struct TaskService {
    connection_pool: Pool<ConnectionManager<PgConnection>>
}

//TODO: what should be in the impl
unsafe impl Send for TaskService {}

unsafe impl Sync for TaskService {}

const TASK_LIMIT: i64 = 25;

impl TaskService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> TaskService {
        TaskService {
            connection_pool: pool
        }
    }

    pub fn create(&self, task: &Task) -> Result<usize, String> {
        self.conn().and_then(|c| {
            super::diesel::insert_into(tasks)
                .values(task)
                .execute(&*c)
                .map_err(TaskService::to_string)
        })
    }

    pub fn delete(&self, task_id: i32) -> Result<usize, String> {
        self.conn().and_then(|c|
            super::diesel::delete(tasks.filter(id.eq(task_id)))
                .execute(&*c)
                .map_err(TaskService::to_string)
        )
    }

    pub fn get_tasks(&self, _list: &str, done: bool, date: Option<NaiveDateTime>) -> Result<Vec<Task>, String> {
        self.conn().and_then(|c| {
            let query = tasks
                .filter(list.eq(_list.to_string()))
                .filter(completed.eq(done))
                .limit(TASK_LIMIT)
                .into_boxed();

            let with_due_date = match date {
                Some(d) => query.filter(due.ge(d)),
                _ => query,
            };

            with_due_date
                .load::<Task>(&*c)
                .map_err(TaskService::to_string)
        })
    }

    pub fn complete(&self, _id: i32, done: bool) -> Result<usize, String> {
        self.conn().and_then(|c|
            super::diesel::update(tasks.filter(id.eq(_id)))
                .set(completed.eq(done))
                .execute(&*c)
                .map_err(TaskService::to_string)
        )
    }

    fn to_string<E>(e: E) -> String where E: error::Error {
        e.to_string()
    }

    fn conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, String> {
        self.connection_pool.get().map_err(TaskService::to_string)
    }
}