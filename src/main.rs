use dotenvy::dotenv;

use api;
use rocket::launch;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    api::rocket()
}