use std::future::Future;

pub enum Error {
    ReqwestError(reqwest::Error),
    ResponseShouldBeEmpty,
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait FutureResult<T> = Future<Output = Result<T>>;
