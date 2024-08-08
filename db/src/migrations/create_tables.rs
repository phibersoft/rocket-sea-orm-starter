use sea_orm::{ConnectionTrait, DbConn, EntityTrait, Schema, Statement};

use crate::tables::Task;

async fn create_table<E>(db: &DbConn, entity: E)
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let statement = builder.build(schema.create_table_from_entity(entity).if_not_exists());

    match db.execute(statement).await {
        Ok(_) => {}
        Err(e) => println!("Error while creating {}: {}", entity.table_name(), e)
    }
}

pub async fn create_tables(db: &DbConn) {
    let builder = db.get_database_backend();
    let statement = Statement { db_backend: builder, values: None, sql: "SELECT 'CREATE DATABASE rocket_starter' WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'rocket_starter')\\gexec".to_string() };
    let _ = db.execute(statement).await;
    println!("create_tables running...");
    create_table(db, Task).await;
}