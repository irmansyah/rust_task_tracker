// use crate::utils::utils_setup::{setup, spawn_app};
use tasktracker_backend::adapters::api::tasks::tasks_presenters::TaskPresenter;

use crate::utils::utils_setup::{setup, spawn_app};

#[actix_rt::test]
async fn test_should_return_multiple_results() {
    // setup
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    // given the "all tasks" route

    // when getting
    let response = reqwest::get(&format!("{}/api/v1/tasks/", &api_address)).await.expect("Failed to execute request.");

    // then expect 3 results (inserted in db)
    assert!(response.status().is_success());

    let content_json = response.json::<Vec<TaskPresenter>>().await.unwrap();

    assert_eq!(content_json.len(), 3);
    assert_eq!(content_json[0].title, "Forty-five percent of U.S. tasks sleep in their owner's bed");
    assert_eq!(content_json[0].task_id, "id1".to_string());
}

#[actix_rt::test]
async fn test_should_return_one_results_only() {
    // setup
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    // given the "single tasks" route
    let task_id: i8 = 2;

    // when getting
    let response = reqwest::get(&format!("{}/api/v1/tasks/{}", &api_address, &task_id)).await.expect("Failed to execute request.");

    // then expect 1 result (id 2 inserted in db)
    assert!(response.status().is_success());

    let content_json = response.json::<TaskPresenter>().await.unwrap();

    assert_eq!(content_json.title, "Seventy percent of people sign their task's name on their holiday cards");
    assert_eq!(content_json.task_id, "id1".to_string());
}
