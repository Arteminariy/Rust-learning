#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod models;
mod schema;
mod pagination;
mod error;
mod repositories;
mod wrappers;
mod controllers;
mod services;

mod helpers;

use rocket::{Build, Rocket};
use crate::controllers::users::routes;
use crate::helpers::establish_connection_pool::establish_connection_pool;
use crate::repositories::users::UserRepository;
use crate::services::users::UserService;

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
        .mount("/", routes())
}
