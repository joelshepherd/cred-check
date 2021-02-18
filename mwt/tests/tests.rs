use mwt::{Decoder, Encoder};
use orion::aead::SecretKey;

#[test]
fn it_encodes_ints() {
    let secret = SecretKey::default();

    let encoder = Encoder::new(&secret, 3600);
    let decoder = Decoder::new(&secret, 0);

    let sub = 1234;

    let encoded = encoder.encode(&sub).unwrap();
    let decoded = decoder.decode::<i32>(&encoded).unwrap();

    assert_eq!(sub, decoded);
}

#[test]
fn it_encodes_strings() {
    let secret = SecretKey::default();

    let encoder = Encoder::new(&secret, 3600);
    let decoder = Decoder::new(&secret, 0);

    let sub = "text";

    let encoded = encoder.encode(&sub).unwrap();
    let decoded = decoder.decode::<String>(&encoded).unwrap();

    assert_eq!(sub, decoded);
}

#[test]
fn it_encodes_structs() {
    let secret = SecretKey::default();

    let encoder = Encoder::new(&secret, 3600);
    let decoder = Decoder::new(&secret, 0);

    #[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq, Debug)]
    struct TestSub {
        id: i32,
    }

    let sub = TestSub { id: 1234 };

    let encoded = encoder.encode(&sub).unwrap();
    let decoded = decoder.decode::<TestSub>(&encoded).unwrap();

    assert_eq!(sub, decoded);
}
