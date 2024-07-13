use thiserror::Error;

use crate::utils::request_reader;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Http request is empty")]
    EmptyHttpRequest,
    #[error("{0}")]
    ReadError(#[from] request_reader::Error),
    #[error("Malformed http request")]
    MalformedRequest,
    #[error("Http request unexpectedly ended")]
    RequestUnexpectedlyEnded,
    #[error("Invalid header {0}: \"{1}\"")]
    InvalidHeader(String, String),
}

pub type Result<T> = std::result::Result<T, Error>;
