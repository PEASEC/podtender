use crate::error::{PodtenderError, Result};
use crate::podman_service::network_internals::PodmanServiceResponseBytes;
use crate::podtender_errors::PodmanErrorResponse;
use crate::podtender_errors::RequestError;
use crate::PODMAN_API_VERSION;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
#[cfg(feature = "enable-tracing")]
use tracing::{debug, warn};

/// Creates the endpoint part of the request. Change `PODMAN_API_VERSION` to switch to a different
/// version of the api (assuming the endpoints didn't change).
pub(crate) fn create_endpoint(endpoint: &str) -> String {
    format!("{}{}", PODMAN_API_VERSION, endpoint)
}

/// This method deserializes the response from the podman service into the expected type `T`. Type `T`
/// is provided by the api call function.
pub(crate) fn deserialize_service_response<T: DeserializeOwned + std::fmt::Debug>(
    service_response: PodmanServiceResponseBytes,
) -> Result<T> {
    #[cfg(feature = "enable-tracing")]
    {
        let tmp = String::from_utf8(service_response.body.to_vec());
        debug!(raw_response = ?tmp);
    }

    // Deserialization is tried first since some responses don't come with success status codes
    // but are expected and should be deserialized. Only after deserialization has failed, the
    // status code get's checked.

    let response_deserializer =
        &mut serde_json::Deserializer::from_slice(service_response.body.as_ref());

    // this check is needed since podman returns the same json schema on success and on error with code 409
    match serde_path_to_error::deserialize(response_deserializer) {
        // deserialization worked
        Ok(result) => {
            #[cfg(feature = "enable-tracing")]
            debug!(deserialized_response = ?result);
            Ok(result)
        }
        Err(error) => {
            // deserialization didn't work but the http response code indicates success -> issue with deserialization
            if service_response.status_code.is_success() {
                #[cfg(feature = "enable-tracing")]
                warn!(
                    ?error,
                    "Status code indicates success but deserialization failed"
                );

                Err(PodtenderError::from(error))
            }
            // deserialization didn't work and http response code doesn't indicate success -> podman returned an error
            else {
                Err(handle_service_response_error(service_response))
            }
        }
    }
}

/// Some podman api endpoints only respond with a status code (2xx) in case of success. This method
/// checks whether the response indicates success or failure. Failure get's handled by [`handle_damon_response_error()`].
pub(crate) fn check_service_response_for_error(
    service_response: PodmanServiceResponseBytes,
) -> Result<()> {
    if service_response.status_code.is_success() {
        Ok(())
    } else {
        Err(handle_service_response_error(service_response))
    }
}

/// Differentiates between an error response from podman to a successful request and an error response
/// from podman to a malformed/failed request.
pub(crate) fn handle_service_response_error(
    service_response: PodmanServiceResponseBytes,
) -> PodtenderError {
    // podman returned an error to a successful request
    if let Ok(podman_error) =
        serde_json::from_slice::<PodmanErrorResponse>(service_response.body.as_ref())
    {
        #[cfg(feature = "enable-tracing")]
        warn!(?podman_error);

        podman_error.into()
    } else {
        // podman returned an error to an unsuccessful request

        // parse error message returned from request (no JSON)
        let message = {
            match String::from_utf8(service_response.body.to_vec()) {
                Ok(message) => message,
                Err(error) => return error.into(),
            }
        };
        let request_error = RequestError {
            message,
            response_code: service_response.status_code.as_u16(),
        };

        #[cfg(feature = "enable-tracing")]
        warn!(?request_error,);

        request_error.into()
    }
}

/// Convert `Option<HashMap<String, Vec<String>>>` to `Option<String>`.
/// This is needed since the API expects `map[string][]string` as query parameter and directly
/// serializing doesn't provide this format.
pub(crate) fn convert_from_map_to_json_string(
    map: Option<HashMap<String, Vec<String>>>,
) -> Result<Option<String>> {
    Ok(if let Some(map) = map {
        Some(serde_json::to_string(&map)?)
    } else {
        None
    })
}
