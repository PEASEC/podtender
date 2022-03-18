use crate::error::Result;
use crate::networks::parameter_types::*;
use crate::networks::response_types::*;
use crate::podman_service::PodmanService;
use crate::utils;
use std::convert::TryInto;
#[cfg(feature = "enable-tracing")]
use tracing::instrument;

/// Network operations.
#[derive(Debug)]
pub struct Networks<'service> {
    podman_service: &'service PodmanService,
}

impl<'service> Networks<'service> {
    pub(crate) fn new(podman_service: &'service PodmanService) -> Self {
        Networks { podman_service }
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/NetworkDeleteLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Networks.remove"))]
    pub async fn remove(
        &self,
        parameter: RemoveNetworkParameter,
    ) -> Result<Vec<RemoveNetworkResponse>> {
        let endpoint =
            utils::create_endpoint(&format!("/libpod/networks/{}", parameter.network_name));
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .delete_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/NetworkConnectLibpod>
    #[cfg_attr(
        feature = "enable-tracing",
        instrument(name = "Networks.connect_container")
    )]
    pub async fn connect_container(&self, parameter: ConnectContainerParameter) -> Result<()> {
        let endpoint = utils::create_endpoint(&format!(
            "/libpod/networks/{}/connect",
            parameter.network_name
        ));
        let body = serde_json::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, Some(body))
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/NetworkDisconnectLibpod>
    #[cfg_attr(
        feature = "enable-tracing",
        instrument(name = "Networks.disconnect_container")
    )]
    pub async fn disconnect_container(
        &self,
        parameter: DisconnectContainerParameter,
    ) -> Result<()> {
        let endpoint = utils::create_endpoint(&format!(
            "/libpod/networks/{}/disconnect",
            parameter.network_name
        ));
        let body = serde_json::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, Some(body))
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/NetworkExistsLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Networks.exists"))]
    pub async fn exists(&self, parameter: NetworkExistsParameter) -> Result<()> {
        let endpoint = utils::create_endpoint(&format!(
            "/libpod/networks/{}/exists",
            parameter.network_name
        ));
        let service_response = self
            .podman_service
            .get_request(&endpoint, None, None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/NetworkInspectLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Networks.inspect"))]
    pub async fn inspect(
        &self,
        parameter: InspectNetworkParameter,
    ) -> Result<InspectNetworkResponse> {
        let endpoint =
            utils::create_endpoint(&format!("/libpod/networks/{}/json", parameter.network_name));
        let service_response = self
            .podman_service
            .get_request(&endpoint, None, None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/NetworkCreateLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Networks.create"))]
    pub async fn create(&self, parameter: CreateNetworkParameter) -> Result<CreateNetworkResponse> {
        let endpoint = utils::create_endpoint("/libpod/networks/create");
        let body = serde_json::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, Some(body))
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/NetworkListLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Networks.list"))]
    pub async fn list(
        &self,
        parameter: ListNetworksParameter,
    ) -> Result<Vec<ListNetworksResponseEntry>> {
        let parameter: ListNetworksParameterQuery = parameter.try_into()?;
        let endpoint = utils::create_endpoint("/libpod/networks/json");

        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .get_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/NetworkPruneLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Networks.prune"))]
    pub async fn prune(
        &self,
        parameter: PruneNetworksParameter,
    ) -> Result<Vec<PruneNetworksResponseEntry>> {
        let parameter: PruneNetworksParameterQuery = parameter.try_into()?;
        let endpoint = utils::create_endpoint("/libpod/networks/prune");

        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }
}
