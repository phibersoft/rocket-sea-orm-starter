use db::{Database, run_migrations};
use db::pool;
use rocket::{Build, catch, catchers, Request, Rocket};
use rocket::fairing::AdHoc;
use rocket::http::Status;

use crate::types::ErrorResponse;

mod routes;
mod types;

#[catch(404)]
fn not_found() -> ErrorResponse {
    ErrorResponse::new(Some("Not found."), Some(404))
}

#[catch(422)]
fn unprocessable_entity() -> ErrorResponse {
    ErrorResponse::new(Some("You sent wrong type of data. Check your body."), Some(422))
}

#[catch(default)]
fn default_catcher(status: Status, _: &Request) -> ErrorResponse {
    ErrorResponse::new(Some("Something went wrong."), Some(status.code as usize))
}

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(pool::Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/task", routes::task::routes())
        .mount("/progress", routes::progress::routes())
        .register("/", catchers![not_found, unprocessable_entity, default_catcher])
}
