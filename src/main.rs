#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod controller;
mod models;
mod schema;
mod repository;
mod service;

mod pagination;

mod error;

mod hoc;

use rocket::{Build, Rocket};
use repository::UserRepository;
use rust_test_ms::establish_connection_pool;
use service::UserService;

#[launch]
fn rocket() -> Rocket<Build> {
    let pool = establish_connection_pool();

    let user_service = UserService {
        repo: UserRepository {
            pool,
        },
    };

    rocket::build()
        .manage(user_service)
        .mount("/", controller::routes())
}
