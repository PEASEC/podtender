//! <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#tag/system>

mod api_call_functions;
/// Parameter types for system operations.
pub mod parameter_types;
/// Response types for system operations.
pub mod response_types;

pub use api_call_functions::*;
