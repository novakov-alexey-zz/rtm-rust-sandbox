use core::models::Task;
use diesel::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use super::chrono::DateTime;
use super::chrono::Local;

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

    pub fn get_tasks(&self, _list: String, _completed: bool, date: DateTime<Local>) -> Result<Vec<Task>, Error> {
        use core::schema::tasks::dsl::*;
        println!("{:?}", date);

        tasks
            .filter(list.eq(_list))
            .filter(completed.eq(_completed))
            .limit(25)
            .load::<Task>(&self.connection)
    }

    pub fn complete(&self, _id: i32, done: bool) -> Result<usize, Error> {
        use super::schema::tasks::dsl::*;

        super::diesel::update(tasks.filter(id.eq(_id)))
            .set(completed.eq(done))
            .execute(&self.connection)
    }
}