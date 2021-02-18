use serde_json::{json, Value};
use warp::hyper::{body::Bytes, Response};

/// Prepare a json response for snapshot
#[allow(dead_code)]
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

#[derive(serde::Deserialize)]
struct TokenReply {
    token: String,
}

#[allow(dead_code)]
pub async fn login() -> String {
    let res = warp::test::request()
        .method("POST")
        .path("/login")
        .body(r#"{"username": "test"}"#)
        .reply(&api::init().await)
        .await;

    let reply = serde_json::from_slice::<TokenReply>(&res.into_body()).unwrap();
    reply.token
}
