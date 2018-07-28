extern crate chrono;
extern crate core;
extern crate rocket;
extern crate rocket_contrib;
extern crate rtm;
extern crate openssl;

use rtm::core::service::TaskService;
use rtm::create_db_pool;
use rtm::routes::mount_routes;

fn main() {
    let error = mount_routes(TaskService::new(create_db_pool())).launch();
    drop(error);
}
