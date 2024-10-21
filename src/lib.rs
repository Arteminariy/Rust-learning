use dotenvy::dotenv;
use std::env;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub fn establish_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}