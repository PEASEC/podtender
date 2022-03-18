//! <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#tag/networks>

mod api_call_functions;

/// Reponse types for network operations.
pub mod parameter_types;
/// Response types for network operations.
pub mod response_types;

pub use api_call_functions::*;
