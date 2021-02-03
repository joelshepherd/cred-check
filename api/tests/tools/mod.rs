use serde_json::{json, Map, Value};
use warp::hyper::{body::Bytes, Response};

/// Prepare a JSON response for snapshot
pub fn json_response(response: Response<Bytes>) -> Value {
    match serde_json::from_slice::<Value>(&response.into_body()) {
        Ok(mut value) => {
            if let Some(map) = value.as_object_mut() {
                replace_keys(map);
            }
            value
        }
        Err(_) => Value::Null,
    }
}

fn replace_keys(object: &mut Map<String, Value>) {
    for (key, value) in object.iter_mut() {
        match value.as_object_mut() {
            Some(v) => replace_keys(v),
            None => {
                if key == "id" {
                    *value = json!("[id]");
                }
            }
        }
    }
}
