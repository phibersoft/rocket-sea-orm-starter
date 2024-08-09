use db::{Connection, pool};
use db::sea_orm::{ActiveModelTrait, EntityTrait};
use db::tables::{Task, task};
use rocket::{get, post, Route, routes};
use rocket::serde::json::Json;

use crate::types::{Authenticated, JsonResponse};

#[get("/")]
async fn list(conn: Connection<pool::Db>, user: Authenticated) -> JsonResponse<Vec<task::Model>> {
    let user = user?;
    println!("Logged in user: {}", user.claims.email);

    let db = conn.into_inner();
    let rows = Task::find().all(&db).await.unwrap();
    Ok(Json(rows))
}

#[post("/", format = "json", data = "<input_data>")]
async fn new(conn: Connection<pool::Db>, input_data: Json<task::InputData>) -> JsonResponse<task::Model> {
    let data = input_data.into_inner();

    let task = task::Model::new(data);
    let task: task::Model = task.insert(&conn.into_inner()).await.unwrap();

    Ok(Json(task))
}

pub fn routes() -> Vec<Route> {
    routes![list, new]
}