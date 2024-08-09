use std::env;
use std::time::Duration;

use async_trait::async_trait;
use rocket_db_pools::Database;
use rocket_db_pools::rocket::figment::Figment;
use sea_orm::ConnectOptions;

#[derive(Database, Debug)]
#[database("rocket_starter")]
pub struct Db(SeaOrmPool);

#[derive(Debug, Clone)]
pub struct SeaOrmPool {
    pub conn: sea_orm::DatabaseConnection,
}

#[async_trait]
impl rocket_db_pools::Pool for SeaOrmPool {
    type Connection = sea_orm::DatabaseConnection;

    type Error = sea_orm::DbErr;

    async fn init(_figment: &Figment) -> Result<Self, Self::Error> {
        let profile = env::var("ROCKET_PROFILE");
        let is_testing = profile.is_ok() && profile.unwrap() == "test";

        let connection_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

        let mut options = ConnectOptions::new(connection_url);
        options.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8));


        if is_testing {
            options.sqlx_logging(false);
        }


        let conn = sea_orm::Database::connect(options).await.unwrap();
        Ok(SeaOrmPool { conn })
    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        Ok(self.conn.clone())
    }

    async fn close(&self) {
        let _ = &self.get().await.unwrap().close();
    }
}