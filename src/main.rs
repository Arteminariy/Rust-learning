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

mod traits;

use rocket::{Build, Rocket};
use crate::controllers::auth::auth_routes;
use crate::controllers::users::user_routes;
use crate::helpers::establish_connection_pool::establish_connection_pool;
use crate::repositories::users::UserRepository;
use crate::services::auth::AuthService;
use crate::services::users::UserService;

#[launch]
fn rocket() -> Rocket<Build> {
    let pool = establish_connection_pool();

    let user_service = UserService {
        repo: UserRepository {
            pool: pool.clone(),
        },
    };

    let auth_service = AuthService {
        user_service: UserService {
            repo: UserRepository {
                pool: pool.clone(),
            },
        }
    };

    rocket::build()
        .manage(user_service)
        .manage(auth_service)
        .mount("/", [user_routes(), auth_routes()].concat())
}
