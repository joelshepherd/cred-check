mod tools;

use api::init;
use insta::assert_json_snapshot;
use tools::json_response;
use warp::http::StatusCode;
use warp::test::request;

#[tokio::test]
async fn test_create() {
    let api = init().await;

    let res = request()
        .method("POST")
        .path("/user")
        .body(
            r#"{
                "name": "Tester",
                "username": "tester"
            }"#,
        )
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);
    assert_json_snapshot!(json_response(res));
}
