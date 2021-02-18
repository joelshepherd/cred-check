use crate::{now, SecretKey, Token};
use orion::aead;

#[derive(Debug)]
pub enum DecodeError {
    DecryptError,
    DeserializeError,
    NotValidYet,
    NotValidAnymore,
}

#[derive(Clone, Copy)]
pub struct Decoder<'a> {
    leeway: u64,
    secret: &'a SecretKey,
}

impl<'a> Decoder<'a> {
    pub fn new(secret: &'a SecretKey, leeway: u64) -> Self {
        Self { leeway, secret }
    }

    /// Decode a token
    pub fn decode<T: serde::Serialize + serde::de::DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<T, DecodeError> {
        let token = base64::decode(token).map_err(|_| DecodeError::DecryptError)?;
        let token = aead::open(self.secret, &token).map_err(|_| DecodeError::DecryptError)?;
        let token: Token<T> =
            bincode::deserialize(&token).map_err(|_| DecodeError::DeserializeError)?;

        let time = now();
        if (time + self.leeway) < token.nbf {
            return Err(DecodeError::NotValidYet);
        }
        if (time - self.leeway) > token.exp {
            return Err(DecodeError::NotValidAnymore);
        }

        Ok(token.sub)
    }
}
