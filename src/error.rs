use crate::podtender_errors::PodmanErrorResponse;
use crate::podtender_errors::RequestError;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PodtenderError {
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[error(transparent)]
    HyperHttpError(#[from] hyper::http::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeJsonErrorWithPath(#[from] serde_path_to_error::Error<serde_json::Error>),
    #[error(transparent)]
    SerdeQsError(#[from] serde_qs::Error),
    #[error(transparent)]
    PodmanErrorResponse(#[from] PodmanErrorResponse),
    #[error(transparent)]
    RequestError(#[from] RequestError),
    #[error(transparent)]
    MyJsonCodecError(#[from] asynchronous_codec::JsonCodecError),
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, PodtenderError>;
