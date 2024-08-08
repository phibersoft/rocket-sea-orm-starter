use api;
use rocket::tokio::runtime;

fn main() {
    let _ = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(api::rocket().launch());
}