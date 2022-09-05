#![allow(clippy::module_inception)]
#![doc = include_str!("../README.md")]
pub mod containers;
pub mod error;
pub mod images;
pub mod networks;
pub mod podman_service;
pub mod pods;
pub mod podtender_errors;
pub mod system;
mod utils;
pub mod volumes;

#[cfg(any(test, feature = "examples"))]
pub mod example_values_trait;

/// The currently targeted podman api version. Will be used in the requests to the api as follows:
/// `http://d/{PODMAN_API_VERSION}/libpod/...`
static PODMAN_API_VERSION: &str = "/v4.2.0";
