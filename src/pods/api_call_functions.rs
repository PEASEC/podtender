use crate::error::Result;
use crate::podman_service::PodmanService;
use crate::pods::parameter_types::*;
use crate::pods::response_types::*;
use crate::utils;
use futures::Stream;
use std::convert::TryInto;
#[cfg(feature = "enable-tracing")]
use tracing::instrument;

/// Pod operations.
#[derive(Debug)]
pub struct Pods<'service> {
    podman_service: &'service PodmanService,
}

impl<'service> Pods<'service> {
    pub(crate) fn new(podman_service: &'service PodmanService) -> Self {
        Pods { podman_service }
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodCreateLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.create"))]
    pub async fn create(&self, parameter: CreatePodParameter) -> Result<CreatePodResponse> {
        let endpoint = utils::create_endpoint("/libpod/pods/create");
        let body = serde_json::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, Some(body))
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodDeleteLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.remove"))]
    pub async fn remove(&self, parameter: RemovePodParameter) -> Result<RemovePodResponse> {
        let endpoint = format!(
            "{}{}",
            utils::create_endpoint("/libpod/pods/"),
            parameter.pod_name
        );
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .delete_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodExistsLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.exists"))]
    pub async fn exists(&self, parameter: PodExistsParameter) -> Result<()> {
        let endpoint = format!(
            "{}{}/exists",
            utils::create_endpoint("/libpod/pods/"),
            parameter.pod_name
        );
        let service_response = self
            .podman_service
            .get_request(&endpoint, None, None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodInspectLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.inspect"))]
    pub async fn inspect(&self, parameter: InspectPodParameter) -> Result<InspectPodResponse> {
        let endpoint = format!(
            "{}{}/json",
            utils::create_endpoint("/libpod/pods/"),
            parameter.pod_name
        );
        let service_response = self
            .podman_service
            .get_request(&endpoint, None, None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodKillLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.kill"))]
    pub async fn kill(&self, parameter: KillPodParameter) -> Result<KillPodResponse> {
        let endpoint = format!(
            "{}{}/kill",
            utils::create_endpoint("/libpod/pods/"),
            parameter.pod_name
        );
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodPauseLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.pause"))]
    pub async fn pause(&self, parameter: PausePodParameter) -> Result<PausePodResponse> {
        let endpoint = format!(
            "{}{}/pause",
            utils::create_endpoint("/libpod/pods/"),
            parameter.pod_name
        );
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodRestartLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.restart"))]
    pub async fn restart(&self, parameter: RestartPodParameter) -> Result<RestartPodResponse> {
        let endpoint = format!(
            "{}{}/restart",
            utils::create_endpoint("/libpod/pods/"),
            parameter.pod_name
        );
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodStartLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.start"))]
    pub async fn start(&self, parameter: StartPodParameter) -> Result<StartPodResponse> {
        let endpoint = format!(
            "{}{}/start",
            utils::create_endpoint("/libpod/pods/"),
            parameter.pod_name
        );
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodStopLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.stop"))]
    pub async fn stop(&self, parameter: StopPodParameter) -> Result<StopPodResponse> {
        let endpoint = format!(
            "{}{}/stop",
            utils::create_endpoint("/libpod/pods/"),
            parameter.pod_name
        );
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodTopLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.list_processes"))]
    pub async fn list_processes(
        &self,
        parameter: ListPodProcessesParameter,
    ) -> Result<ListPodProcessesResponse> {
        let endpoint = format!(
            "{}{}/top",
            utils::create_endpoint("/libpod/pods/"),
            parameter.pod_name
        );

        let parameter = ListPodProcessesParameter {
            stream: Some(false),
            ..parameter
        };

        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .get_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodTopLibpod>
    #[cfg_attr(
        feature = "enable-tracing",
        instrument(name = "Pods.list_processes_streaming")
    )]
    pub async fn list_processes_streaming(
        &self,
        parameter: ListPodProcessesParameter,
    ) -> Result<impl Stream<Item = Result<ListPodProcessesResponse>>> {
        let endpoint = format!(
            "{}{}/top",
            utils::create_endpoint("/libpod/pods/"),
            parameter.pod_name
        );
        let parameter = ListPodProcessesParameter {
            stream: Some(true),
            ..parameter
        };
        let query = serde_qs::to_string(&parameter)?;

        let (_status_code, result_stream, _header_map) = self
            .podman_service
            .get_json_stream(&endpoint, Some(query), None, None)
            .await?;
        Ok(result_stream)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodUnpauseLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.unpause"))]
    pub async fn unpause(&self, parameter: UnpausePodParameter) -> Result<UnpausePodResponse> {
        let endpoint = format!(
            "{}{}/unpause",
            utils::create_endpoint("/libpod/pods/"),
            parameter.pod_name
        );
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodListLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.list"))]
    pub async fn list(&self, parameter: ListPodsParameter) -> Result<Vec<ListPodsResponseEntry>> {
        let parameter: ListPodsParameterQuery = parameter.try_into()?;
        let endpoint = utils::create_endpoint("/libpod/pods/json");

        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .get_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodPruneLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.prune"))]
    pub async fn prune(&self) -> Result<Vec<PrunePodsResponse>> {
        let endpoint = utils::create_endpoint("/libpod/pods/prune");
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/PodStatsAllLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Pods.stats"))]
    pub async fn stats(
        &self,
        parameter: PodStatsParameter,
    ) -> Result<impl Stream<Item = Result<Vec<PodStatsResponse>>>> {
        let endpoint = format!("{}stats", utils::create_endpoint("/libpod/pods/"),);

        // Start workaround cause podman only supports query arrays in this format:
        // containers=container1&containers=container2
        let query = {
            use serde::Serialize;
            use serde_with::skip_serializing_none;

            #[skip_serializing_none]
            #[derive(Serialize, Debug)]
            struct TempQuery {
                all: Option<bool>,
            }
            let temp_query = TempQuery { all: parameter.all };

            if parameter.names_or_ids.is_none() {
                serde_qs::to_string(&temp_query)?
            } else {
                let temp_query = serde_qs::to_string(&temp_query)?;
                if parameter.names_or_ids.as_ref().unwrap().is_empty() {
                    temp_query
                } else {
                    #[skip_serializing_none]
                    #[derive(Serialize, Debug)]
                    struct TempNamesOrIDs {
                        #[serde(rename = "namesOrIDs")]
                        names_or_ids: String,
                    }
                    let mut names_or_ids_string = String::new();
                    for names_or_ids in parameter.names_or_ids.unwrap() {
                        let temp_name_or_ids = TempNamesOrIDs { names_or_ids };
                        names_or_ids_string.push_str(&serde_qs::to_string(&temp_name_or_ids)?);
                        names_or_ids_string.push('&');
                    }
                    names_or_ids_string.push_str(&temp_query);
                    names_or_ids_string
                }
            }
        };

        //end workaround

        let (_status_code, result_stream, _header_map) = self
            .podman_service
            .get_json_stream(&endpoint, Some(query), None, None)
            .await?;
        Ok(result_stream)
    }
}
