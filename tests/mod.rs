use std::env;

mod task;
pub mod common;

#[rocket::async_test]
async fn main() {
    env::set_var("ROCKET_PROFILE", "test");
}