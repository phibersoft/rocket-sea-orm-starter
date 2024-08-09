use db::{Connection, pool};
use db::sea_orm::EntityTrait;
use db::tables::Progress;
use db::tables::progress::Model;
use rocket::{get, Route, routes};
use rocket::serde::json::Json;

use crate::types::JsonResponse;

#[get("/")]
async fn list(conn: Connection<pool::Db>) -> JsonResponse<Vec<Model>> {
    let db = conn.into_inner();
    let rows = Progress::find().all(&db).await.unwrap();
    Ok(Json(rows))
}

pub fn routes() -> Vec<Route> {
    routes![list]
}