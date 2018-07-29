extern crate chrono;
extern crate core;
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate openssl;
extern crate r2d2;
extern crate rocket;
extern crate rocket_contrib;
extern crate rtm;

use rtm::{create_db_pool, establish_connection};
use rtm::core::service::TaskService;
use rtm::routes::mount_routes;

embed_migrations!();

fn main() {
    let c = establish_connection();
    embedded_migrations::run_with_output(&c, &mut std::io::stdout())
        .expect("Failed to run database migration");

    let error = mount_routes(TaskService::new(create_db_pool())).launch();
    drop(error);
}
