use crate::{now, SecretKey, Token};
use orion::aead;

#[derive(Debug)]
pub enum EncodeError {
    EncryptError,
    SerializeError,
}

pub struct Encoder<'a> {
    expiry: u64,
    secret: &'a SecretKey,
}

impl<'a> Encoder<'a> {
    pub fn new(secret: &'a SecretKey, expiry: u64) -> Self {
        Self { expiry, secret }
    }

    /// Encode a token
    pub fn encode<T: serde::Serialize>(&self, sub: &T) -> Result<String, EncodeError> {
        let time = now();

        let token = Token {
            nbf: time,
            exp: time + self.expiry,
            sub,
        };
        let token = bincode::serialize(&token).map_err(|_| EncodeError::SerializeError)?;
        let token = aead::seal(self.secret, &token).unwrap(); //.map_err(|_| EncodeError::EncryptError)?;
        let token = base64::encode(token);

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encodes() {
        let secret = SecretKey::default();
        let encoder = Encoder::new(&secret, 3600);
        encoder.encode(&1234).unwrap();
    }
}
