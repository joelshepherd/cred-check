mod tools;

use api::init;
use warp::http::StatusCode;

#[tokio::test]
async fn test_login() {
    let api = init().await;

    let res = warp::test::request()
        .method("POST")
        .path("/login")
        .body(r#"{"username": "test"}"#)
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_signup() {
    let api = init().await;

    let res = warp::test::request()
        .method("POST")
        .path("/signup")
        .body(
            r#"{
                "name": "New Test",
                "username": "new_test"
            }"#,
        )
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);
}
