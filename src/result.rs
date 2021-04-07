use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum Error {
    Timeout,
    DecodeError(String),
    RequestError(String),
    ServiceError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<R, E = Error> = std::result::Result<R, E>;

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::Timeout
        } else if err.is_decode() {
            Error::DecodeError(err.to_string())
        } else {
            Error::RequestError(err.to_string())
        }
    }
}

impl From<telegram_bot::Error> for Error {
    fn from(err: telegram_bot::Error) -> Self {
        Self::ServiceError(err.to_string())
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::ServiceError(err.to_string())
    }
}
