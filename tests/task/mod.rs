use db::tables::task;
use rocket::http::ContentType;
use rocket::serde::json::json;

use crate::common::{Client, TestContext};

#[rocket::async_test]
async fn main() {
    let test_context = TestContext::init("task").await;
    get(&test_context.client, 0).await;
    post(&test_context.client).await;
    get(&test_context.client, 1).await;
    TestContext::tear_down(&test_context).await;
}

async fn get(client: &Client, expected_len: usize) {
    let response = client
        .get("/task")
        .header(ContentType::JSON)
        .dispatch()
        .await
        .into_json::<Vec<task::Model>>()
        .await;

    let task_vec: Vec<task::Model> = response.unwrap_or_else(|| vec![]);
    assert_eq!(task_vec.len(), expected_len);
}

async fn post(client: &Client) {
    let input = task::InputData {
        title: "abc".to_string()
    };

    let response = client
        .post("/task")
        .header(ContentType::JSON)
        .body(json!(input).to_string())
        .dispatch()
        .await
        .into_json::<task::Model>()
        .await;

    assert_eq!(response.unwrap().title, input.title);
}