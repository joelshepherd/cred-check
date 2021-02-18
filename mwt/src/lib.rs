mod decoder;
mod encoder;

pub use decoder::Decoder;
pub use encoder::Encoder;
pub use orion::aead::SecretKey;
use std::time;

#[derive(serde::Deserialize, serde::Serialize)]
struct Token<T: serde::Serialize> {
    nbf: u64,
    exp: u64,
    sub: T,
}

fn now() -> u64 {
    time::SystemTime::now()
        .duration_since(time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
