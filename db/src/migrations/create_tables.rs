use sea_orm::{ConnectionTrait, DbConn, EntityTrait, Schema};

use crate::tables::{Progress, Task, User};

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
    println!("create_tables running...");
    create_table(db, Task).await;
    create_table(db, Progress).await;
    create_table(db, User).await;
}