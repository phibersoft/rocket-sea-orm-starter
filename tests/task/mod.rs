use db::tables::task;
use rocket::http::ContentType;

use crate::common::TestContext;

#[rocket::async_test]
async fn get() {
    let test_context = TestContext::init("get_list").await;
    let response = test_context.client
        .get("/list")
        .header(ContentType::JSON)
        .dispatch()
        .await
        .into_json::<Vec<task::Model>>()
        .await;

    let task_vec = response.expect("no tasks");
    assert_eq!(task_vec.len(), 1);
    TestContext::tear_down(&test_context).await;
}
