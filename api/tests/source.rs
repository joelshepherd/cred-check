mod tools;

use api::init;
use insta::assert_json_snapshot;
use tools::json_response;
use warp::http::StatusCode;
use warp::test::request;

#[tokio::test]
async fn test_find() {
    let api = init().await;

    let res = request().path("/source/test.com").reply(&api).await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_json_snapshot!(json_response(res));
}

#[tokio::test]
async fn test_find_by_alternative() {
    let api = init().await;

    let res = request().path("/source/www.test.com").reply(&api).await;

    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_find_not_found() {
    let api = init().await;

    let res = request()
        .path("/source/test.com/not_found")
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_create() {
    let api = init().await;

    let res = request()
        .method("POST")
        .path("/source")
        .body(r#"{ "url": "https://www.wikipedia.org/" }"#)
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);
    assert_json_snapshot!(json_response(res));
}

#[tokio::test]
async fn test_create_alternative() {
    let api = init().await;

    let res_create = request()
        .method("POST")
        .path("/source")
        .body(r#"{ "url": "https://wikipedia.org/" }"#)
        .reply(&api)
        .await;

    assert_eq!(res_create.status(), StatusCode::CREATED);

    let res_canonical = request()
        .path("/source/https://www.wikipedia.org/")
        .reply(&api)
        .await;

    let res_create = json_response(res_create);
    let res_canonical = json_response(res_canonical);

    assert_eq!(
        res_create.as_object().unwrap()["id"],
        res_canonical.as_object().unwrap()["id"],
    )
}

#[tokio::test]
async fn test_create_invalid_url() {
    let api = init().await;

    let res = request()
        .method("POST")
        .path("/source")
        .body(r#"{ "url": "not a url" }"#)
        .reply(&api)
        .await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
