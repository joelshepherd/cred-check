mod tools;

use api::init;
use insta::assert_json_snapshot;
use tools::json_response;
use warp::http::StatusCode;
use warp::test::request;

#[tokio::test]
async fn test_create() {
    let api = init().await;

    tools::seed_source().await;
    tools::seed_user().await;

    let res = request()
        .method("POST")
        .path("/opinion")
        .header("authorization", "seeded")
        .body(
            r#"{
                "source_id": 1,
                "position": true,
                "body": "This opinion is true."
            }"#,
        )
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);
    assert_json_snapshot!(json_response(res));
}

#[tokio::test]
async fn test_create_with_no_auth() {
    let api = init().await;

    tools::seed_source().await;

    // No auth header provided
    let res = request()
        .method("POST")
        .path("/opinion")
        .body(
            r#"{
                "source_id": 1,
                "position": true,
                "body": "This opinion is true."
            }"#,
        )
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    // Invalid token provided
    let res = request()
        .method("POST")
        .path("/opinion")
        .header("authorization", "invalid_value")
        .body(
            r#"{
                "source_id": 1,
                "position": true,
                "body": "This opinion is true."
            }"#,
        )
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
