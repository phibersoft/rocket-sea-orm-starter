use db::{Database, run_migrations};
use db::pool;
use rocket::{Build, Rocket};
use rocket::fairing::AdHoc;

mod routes;

pub fn rocket() -> Rocket<Build> {
    let rocket = rocket::build()
        .attach(pool::Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/task", routes::task::routes());

    rocket
}
