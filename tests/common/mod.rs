use std::env;
use std::path::Path;

use db::sea_orm;
use db::sea_orm::ConnectionTrait;
pub use rocket::local::asynchronous::Client;

pub struct TestContext {
    db_name: String,
    base_url: String,
    pub client: Client,
}

impl TestContext {
    /// Each integration test gets its own database
    /// The DB url is overridden by adding it to the env variables
    /// since the env variables override the values specified in the
    /// Rocket.toml file
    ///
    /// `test_name` should be unique across the test suite
    ///
    pub async fn init(test_name: &str) -> Self {
        env::set_var("ROCKET_PROFILE", "test");

        let full_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

        let url = Path::new(&full_url);
        let base_url = url.parent().unwrap().to_str().unwrap().to_owned();

        let url = format!("{}/postgres", base_url);
        let db_name = format!("rocket_dbtest_{}", test_name);
        let db = sea_orm::Database::connect(&url).await.unwrap();
        let _drop_db_result = db
            .execute(sea_orm::Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
            ))
            .await;

        let _create_db_result = db
            .execute(sea_orm::Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                format!("CREATE DATABASE \"{}\";", db_name),
            ))
            .await;

        let url = format!("{}/{}", base_url, db_name);

        // Override the DB url by adding an env var
        env::set_var("DATABASE_URL", url);

        let rocket = api::rocket();
        let client = Client::untracked(rocket)
            .await
            .expect("valid rocket instance");

        Self {
            db_name,
            base_url,
            client,
        }
    }

    pub async fn tear_down(test_context: &TestContext) {
        let url = format!("{}/postgres", test_context.base_url);
        let db = sea_orm::Database::connect(&url).await.unwrap();

        let _r = db
            .execute(sea_orm::Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                format!(
                    "DROP DATABASE IF EXISTS \"{}\" WITH (FORCE);",
                    test_context.db_name
                ),
            ))
            .await;
    }
}
