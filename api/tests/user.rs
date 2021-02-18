mod tools;

use api::init;
use insta::assert_json_snapshot;
use tools::json_response;
use warp::http::StatusCode;
use warp::test::request;

#[tokio::test]
async fn test_find() {
    let api = init().await;

    let res = request().path("/user/1").reply(&api).await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_json_snapshot!(json_response(res));
}
