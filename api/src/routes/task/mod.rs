use db::{Connection, pool};
use db::sea_orm::EntityTrait;
use db::tables::Task;
use db::tables::task::Model;
use rocket::{get, Route, routes};
use rocket::serde::json::Json;

#[get("/list")]
async fn list(conn: Connection<pool::Db>) -> Json<Vec<Model>> {
    let db = conn.into_inner();
    let rows = Task::find().all(&db).await.unwrap();
    Json(rows)
}

pub fn routes() -> Vec<Route> {
    routes![list]
}