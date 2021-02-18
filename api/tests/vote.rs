mod tools;

use api::init;
use insta::assert_json_snapshot;
use tools::json_response;
use warp::http::StatusCode;
use warp::test::request;

#[tokio::test]
async fn test_create() {
    let api = init().await;

    let token = tools::login().await;

    let res = request()
        .method("POST")
        .path("/vote")
        .header("authorization", token)
        .body(r#"{ "opinion_id": 1 }"#)
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);
    assert_json_snapshot!(json_response(res));
}

#[tokio::test]
async fn test_create_with_no_auth() {
    let api = init().await;

    // No auth header provided
    let res = request()
        .method("POST")
        .path("/vote")
        .body(r#"{ "opinion_id": 1 }"#)
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    // Invalid token provided
    let res = request()
        .method("POST")
        .path("/vote")
        .header("authorization", "invalid_value")
        .body(r#"{ "opinion_id": 1 }"#)
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
