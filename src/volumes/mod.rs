//! <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#tag/volumes>

mod api_call_functions;

/// Parameter types for volume operations.
pub mod parameter_types;
/// Response types for volume operations.
pub mod response_types;

pub use api_call_functions::*;
