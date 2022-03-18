use crate::error::Result;
use crate::podman_service::PodmanService;
use crate::utils;
use crate::volumes::parameter_types::*;
use crate::volumes::response_types::{
    CreateVolumeResponse, InspectVolumeResponse, ListVolumesResponseEntry,
    PruneVolumesResponseEntry,
};
use std::convert::TryInto;
#[cfg(feature = "enable-tracing")]
use tracing::instrument;

/// Volumes operations.
#[derive(Debug)]
pub struct Volumes<'service> {
    podman_service: &'service PodmanService,
}

impl<'service> Volumes<'service> {
    pub(crate) fn new(podman_service: &'service PodmanService) -> Self {
        Volumes { podman_service }
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/VolumeCreateLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Volumes.create"))]
    pub async fn create(&self, parameter: CreateVolumeParameter) -> Result<CreateVolumeResponse> {
        let endpoint = utils::create_endpoint("/libpod/volumes/create");
        let body = serde_json::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, Some(body))
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/VolumeDeleteLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Volumes.remove"))]
    pub async fn remove(&self, parameter: RemoveVolumeParameter) -> Result<()> {
        let endpoint =
            utils::create_endpoint(&format!("/libpod/volumes/{}", parameter.volume_name));
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .delete_request(&endpoint, Some(query), None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/VolumeExistsLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Volumes.exists"))]
    pub async fn exists(&self, param: VolumeExistsParameter) -> Result<()> {
        let endpoint =
            utils::create_endpoint(&format!("/libpod/volumes/{}/exists", param.volume_name));
        let service_response = self
            .podman_service
            .get_request(&endpoint, None, None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/VolumeCreateLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Volumes.inspect"))]
    pub async fn inspect(&self, param: InspectVolumeParameter) -> Result<InspectVolumeResponse> {
        let endpoint =
            utils::create_endpoint(&format!("/libpod/volumes/{}/json", param.volume_name));
        let service_response = self
            .podman_service
            .get_request(&endpoint, None, None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/VolumeListLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Volumes.list"))]
    pub async fn list(
        &self,
        parameter: ListVolumesParameter,
    ) -> Result<Vec<ListVolumesResponseEntry>> {
        let parameter: ListVolumesParameterQuery = parameter.try_into()?;
        let endpoint = utils::create_endpoint("/libpod/volumes/json");

        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .get_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/VolumePruneLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Volumes.prune"))]
    pub async fn prune(
        &self,
        parameter: PruneVolumesParameter,
    ) -> Result<Vec<PruneVolumesResponseEntry>> {
        let parameter: PruneVolumesParameterQuery = parameter.try_into()?;
        let endpoint = utils::create_endpoint("/libpod/volumes/prune");

        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }
}
