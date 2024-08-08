use api;
use rocket::launch;

#[launch]
fn rocket() -> _ {
    api::rocket()
}