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

mod middlewares;

use crate::middlewares::catchers::unauthorized_catcher::unauthorized_catcher;
use rocket::{Build, Rocket};
use crate::helpers::init_database::init_database;
use crate::controllers::auth::auth_routes;
use crate::controllers::roles::roles_routes;
use crate::controllers::users::user_routes;
use crate::helpers::establish_connection_pool::establish_connection_pool;
use crate::middlewares::catchers::unprocessable_entity_catcher::unprocessable_entity_catcher;
use crate::repositories::roles::RolesRepository;
use crate::repositories::users::UserRepository;
use crate::services::auth::AuthService;
use crate::services::roles::RolesService;
use crate::services::users::UserService;

#[launch]
fn rocket() -> Rocket<Build> {
    let pool = establish_connection_pool();

    init_database(pool.clone());

    let user_service = UserService {
        repo: UserRepository {
            pool: pool.clone(),
        },
    };

    let auth_service = AuthService {
        user_repository: UserRepository {
            pool: pool.clone(),
        }
    };

    let roles_service = RolesService {
        repo: RolesRepository {
            pool: pool.clone(),
        }
    };

    rocket::build()
        .manage(user_service)
        .manage(auth_service)
        .manage(roles_service)
        .mount("/", [user_routes(), auth_routes(), roles_routes()].concat())
        .register("/", catchers![unauthorized_catcher, unprocessable_entity_catcher])
}
