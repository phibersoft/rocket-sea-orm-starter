use db::{Database, run_migrations};
use db::pool;
use rocket::{Build, catch, catchers, Request, Rocket};
use rocket::fairing::AdHoc;

mod routes;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(pool::Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/task", routes::task::routes())
        .mount("/progress", routes::progress::routes())
        .register("/", catchers![not_found])
}
