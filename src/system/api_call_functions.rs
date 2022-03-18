use crate::error::Result;
use crate::podman_service::PodmanService;
use crate::system::parameter_types::{EventsParameter, EventsParameterStreamingQuery};
use crate::system::response_types::{Event, GetInfoResponse};
use crate::utils;
use futures::Stream;
use std::convert::TryInto;
#[cfg(feature = "enable-tracing")]
use tracing::instrument;

/// System operations.
#[derive(Debug)]
pub struct System<'service> {
    podman_service: &'service PodmanService,
}

impl<'service> System<'service> {
    pub(crate) fn new(podman_service: &'service PodmanService) -> Self {
        System { podman_service }
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/SystemInfoLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "System.get_info"))]
    pub async fn get_info(&self) -> Result<GetInfoResponse> {
        let endpoint = utils::create_endpoint("/libpod/info");
        let response = self
            .podman_service
            .get_request(&endpoint, None, None, None)
            .await?;
        utils::deserialize_service_response(response)
    }

    /// Filter `image=name_or_id` only seems to apply to image events like pull, push, remove, etc. Not to container events using this image.
    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/SystemEventsLibpod>
    #[cfg_attr(
        feature = "enable-tracing",
        instrument(name = "System.get_events_streaming")
    )]
    pub async fn get_events_streaming(
        &self,
        parameter: EventsParameter,
    ) -> Result<impl Stream<Item = Result<Event>>> {
        let endpoint = utils::create_endpoint("/libpod/events");
        let parameter: EventsParameterStreamingQuery = parameter.try_into()?;
        let query = serde_qs::to_string(&parameter)?;

        let (_status_code, result_stream, _header_map) = self
            .podman_service
            .get_json_stream(&endpoint, Some(query), None, None)
            .await?;
        Ok(result_stream)
    }
}
