use db::{Database, run_migrations};
use db::pool;
use rocket::{Build, catch, catchers, Request, Rocket};
use rocket::fairing::AdHoc;
use rocket::http::Status;

use crate::error::ErrorResponse;

pub mod routes;
pub mod types;
pub mod authentication;
pub mod error;

#[catch(404)]
fn not_found() -> ErrorResponse {
    ErrorResponse::from(("Not found", 404))
}

#[catch(422)]
fn unprocessable_entity() -> ErrorResponse {
    ErrorResponse::from(("You sent wrong type of data. Check your body.", 422))
}

#[catch(default)]
fn default_catcher(status: Status, _: &Request) -> ErrorResponse {
    ErrorResponse::from(("Something went wrong.", status.code as usize))
}

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(pool::Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/task", routes::task::routes())
        .mount("/progress", routes::progress::routes())
        .mount("/auth", routes::auth::routes())
        .register("/", catchers![not_found, unprocessable_entity, default_catcher])
}
