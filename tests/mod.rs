use std::env;

use dotenvy::dotenv;

mod task;
pub mod common;

#[rocket::async_test]
async fn main() {
    dotenv().ok();
    env::set_var("ROCKET_PROFILE", "test");
}