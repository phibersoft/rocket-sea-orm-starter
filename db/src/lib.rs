pub use rocket_db_pools::{Connection, Database};
pub use sea_orm;

use rocket::{Build, fairing, Rocket};

use crate::migrations::create_tables::create_tables;

pub mod tables;
pub mod pool;
pub mod migrations;

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &pool::Db::fetch(&rocket).unwrap().conn;
    create_tables(conn).await;

    Ok(rocket)
}