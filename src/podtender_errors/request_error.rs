use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// Represents an error returned by the podman API to an incorrect request.
/// Example: Requesting an endpoint with POST instead of GET.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct RequestError {
    pub message: String,
    pub response_code: u16,
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "A Request to Podman returned the following error:\n[{}] message: {}",
            self.response_code, self.message
        )
    }
}
impl Error for RequestError {}
