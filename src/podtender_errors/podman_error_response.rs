use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// Represents an error returned by the podman API to a correct request.
/// Wrong parameters can cause this error.
/// Example: Deleting a non existent container results in this error.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct PodmanErrorResponse {
    pub cause: String,
    pub message: String,
    #[serde(rename = "response")]
    pub response_code: u16,
}

impl fmt::Display for PodmanErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Podman returned the following error:\n[{}] cause: {}, message: {}",
            self.response_code, self.cause, self.message
        )
    }
}
impl Error for PodmanErrorResponse {}
