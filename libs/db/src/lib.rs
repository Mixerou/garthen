#[macro_use]
extern crate log;

use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lazy_static::lazy_static;
use r2d2::{Error, ManageConnection};

pub mod schema;

pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        manager.connect().expect("Failed to connect to the database");
        Pool::new(manager).expect("Failed to create database pool")
    };
}

pub fn get_connection() -> Result<DbConnection, Error> {
    POOL.get()
}

pub fn init() {
    info!("Initialize DB");

    lazy_static::initialize(&POOL);
    let connection = &mut get_connection()
        .expect("Failed to get database connection");
    connection.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run pending database migrations");
}
