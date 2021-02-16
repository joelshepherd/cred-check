use serde_json::{json, Map, Value};
use warp::{
    body::json,
    hyper::{body::Bytes, Response},
};

/// Seed a test source with the url `example.com/seeded`
/// TODO: clippy seems to be flagging incorrectly
#[allow(dead_code)]
pub async fn seed_source() {
    warp::test::request()
        .method("POST")
        .path("/source")
        .body(r#"{ "url": "example.com/seeded" }"#)
        .reply(&api::init().await)
        .await;
}

/// Seed a test user with the username `seeded`
/// TODO: clippy seems to be flagging incorrectly
#[allow(dead_code)]
pub async fn seed_user() {
    warp::test::request()
        .method("POST")
        .path("/user")
        .body(
            r#"{
                "name": "Seeded",
                "username": "seeded"
            }"#,
        )
        .reply(&api::init().await)
        .await;
}

/// Seed a test user with the username `seeded`
/// TODO: clippy seems to be flagging incorrectly
#[allow(dead_code)]
pub async fn seed_opinion() {
    seed_source().await;
    seed_user().await;

    warp::test::request()
        .method("POST")
        .path("/opinion")
        .header("authorization", "seeded")
        .body(
            r#"{
                "source_id": 1,
                "position": false,
                "body": "This opinion is false."
            }"#,
        )
        .reply(&api::init().await)
        .await;
}

/// Prepare a json response for snapshot
pub fn json_response(response: Response<Bytes>) -> Value {
    match serde_json::from_slice::<Value>(&response.into_body()) {
        Ok(mut value) => {
            replace_keys(&mut value);
            value
        }
        Err(_) => Value::Null,
    }
}

fn replace_keys(value: &mut Value) {
    match value {
        Value::Array(v) => {
            for item in v {
                replace_keys(item);
            }
        }
        Value::Object(v) => {
            for (key, vv) in v.iter_mut() {
                if key == "id" {
                    *vv = json!("[id]");
                }
                if key == "created_at" {
                    *vv = json!("[timestamp]");
                }
                match &key[..] {
                    "id" => *vv = json!("[id]"),
                    "created_at" => *vv = json!("[created_at]"),
                    _ => replace_keys(vv),
                }
            }
        }
        _ => {}
    }
}
