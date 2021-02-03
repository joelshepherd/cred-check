use api::init;
use warp::http::StatusCode;
use warp::test::request;

#[tokio::test]
async fn test_find() {
    let api = init().await;

    // Insert data
    request()
        .method("POST")
        .path("/source")
        .body(r#"{ "url": "example.com/existing" }"#)
        .reply(&api)
        .await;

    let res = request()
        .path("/source/example.com/existing")
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert!(std::str::from_utf8(res.body())
        .unwrap()
        .contains("example.com/existing"));
}

#[tokio::test]
async fn test_create() {
    let api = init().await;

    let res = request()
        .method("POST")
        .path("/source")
        .body(r#"{ "url": "example.com/new" }"#)
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);
}
