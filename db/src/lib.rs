// hide diesel derive warning
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
use diesel::prelude::*;

pub struct DbConnecting;
impl DbConnecting {
    pub fn establish_connection() -> diesel::pg::PgConnection {
        dotenv::dotenv().ok();
        let s = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        diesel::pg::PgConnection::establish(&s).expect("Failed to create connection.")
    }
    pub fn establish_pool() -> diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>> {
        dotenv::dotenv().ok();
        let s = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(s);
        let pool = diesel::r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        pool
    }
}
pub type DbConn = diesel::pg::PgConnection;
pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;

pub mod models;
pub mod schema;
