use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Internal,
    Invalid,
    NotFound,
    Unauthorised,
}

impl warp::reject::Reject for Error {}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Error {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::Internal,
        }
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(_: validator::ValidationErrors) -> Self {
        Error::Invalid
    }
}
