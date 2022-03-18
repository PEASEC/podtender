use crate::containers::parameter_types::*;
use crate::containers::response_types::*;
use crate::error::Result;
use crate::podman_service::PodmanService;
use crate::podtender_errors::PodmanErrorResponse;
use crate::utils;
use futures::Stream;
use hyper::http;
use std::convert::TryInto;
#[cfg(feature = "enable-tracing")]
use tracing::instrument;

/// Container operations.
#[derive(Debug)]
pub struct Containers<'service> {
    podman_service: &'service PodmanService,
}

impl<'service> Containers<'service> {
    pub(crate) fn new(podman_service: &'service PodmanService) -> Self {
        Containers { podman_service }
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerCreateLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.create"))]
    pub async fn create(
        &self,
        parameter: CreateContainerParameter,
    ) -> Result<CreateContainerResponse> {
        let endpoint = utils::create_endpoint("/libpod/containers/create");
        let body = serde_json::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, Some(body))
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerListLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.list"))]
    pub async fn list(
        &self,
        parameter: ListContainersParameter,
    ) -> Result<Vec<ListContainersResponseEntry>> {
        let parameter: ListContainersParameterQuery = parameter.try_into()?;
        let endpoint = utils::create_endpoint("/libpod/containers/json");

        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .get_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerDeleteLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.delete"))]
    pub async fn delete(
        self,
        parameter: DeleteContainerParameter,
    ) -> Result<Option<Vec<ContainerDeleteResponseEntry>>> {
        let endpoint = format!(
            "{}{}",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .delete_request(&endpoint, Some(query), None, None)
            .await?;
        if service_response.status_code.is_success() {
            if service_response.status_code == http::StatusCode::NO_CONTENT {
                Ok(None)
            } else {
                let response_array: Result<Vec<ContainerDeleteResponseEntry>> =
                    utils::deserialize_service_response(service_response);
                match response_array {
                    Ok(response_array) => Ok(Some(response_array)),
                    Err(err) => Err(err),
                }
            }
        } else {
            let error: PodmanErrorResponse =
                serde_json::from_slice(service_response.body.as_ref())?;
            Err(error.into())
        }
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerStartLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.start"))]
    pub async fn start(&self, parameter: StartContainerParameter) -> Result<()> {
        let endpoint = format!(
            "{}{}/start",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, Some(query), None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerStopLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.stop"))]
    pub async fn stop(&self, parameter: StopContainerParameter) -> Result<()> {
        let endpoint = format!(
            "{}{}/stop",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, Some(query), None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerCheckpointLibpod>
    /// Needs CRIU and root.
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.checkpoint"))]
    pub async fn checkpoint(
        &self,
        parameter: CheckpointContainerParameter,
    ) -> Result<impl Stream<Item = Result<Vec<u8>>>> {
        let endpoint = format!(
            "{}{}/checkpoint",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );

        let query = serde_qs::to_string(&parameter)?;

        let (_status_code, result_stream, _header_map) = self
            .podman_service
            .post_receive_file_chunks_stream(&endpoint, Some(query), None, None)
            .await?;
        Ok(result_stream)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerExistsLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.exists"))]
    pub async fn exists(&self, parameter: ContainerExistsParameter) -> Result<()> {
        let endpoint = format!(
            "{}{}/exists",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let service_response = self
            .podman_service
            .get_request(&endpoint, None, None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerExportLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.export"))]
    pub async fn export(
        &self,
        parameter: ExportContainerParameter,
    ) -> Result<impl Stream<Item = Result<Vec<u8>>>> {
        let endpoint = format!(
            "{}{}/export",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );

        let (_status_code, result_stream, _header_map) = self
            .podman_service
            .get_receive_file_chunks_stream(&endpoint, None, None, None)
            .await?;
        Ok(result_stream)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerHealthcheckLibpod>
    /// Only supported by containers created with docker's format, oci spec doesn't support
    /// healthcheck, see <https://github.com/opencontainers/image-spec/issues/749>
    #[cfg_attr(
        feature = "enable-tracing",
        instrument(name = "Containers.healthcheck")
    )]
    pub async fn healthcheck(
        &self,
        parameter: HealthcheckContainerParameter,
    ) -> Result<HealthcheckContainerResponse> {
        let endpoint = format!(
            "{}{}/healthcheck",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let service_response = self
            .podman_service
            .get_request(&endpoint, None, None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerInitLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.initialize"))]
    pub async fn initialize(&self, parameter: InitializeContainerParameter) -> Result<()> {
        let endpoint = format!(
            "{}{}/init",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, None)
            .await?;
        if service_response.status_code == 304 {
            return Ok(());
        }
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerInspectLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.inspect"))]
    pub async fn inspect(
        &self,
        parameter: InspectContainerParameter,
    ) -> Result<InspectContainerResponse> {
        let endpoint = format!(
            "{}{}/json",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .get_request(&endpoint, Some(query), None, None)
            .await?;

        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerKillLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.kill"))]
    pub async fn kill(&self, parameter: KillContainerParameter) -> Result<()> {
        let endpoint = format!(
            "{}{}/kill",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, Some(query), None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerLogsLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.logs"))]
    pub async fn logs(
        &self,
        parameter: ContainerLogsParameter,
    ) -> Result<impl Stream<Item = Result<String>>> {
        let endpoint = format!(
            "{}{}/logs",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let query = serde_qs::to_string(&parameter)?;
        let (_status_code, result_stream, _header_map) = self
            .podman_service
            .get_lines_stream(&endpoint, Some(query), None, None)
            .await?;
        Ok(result_stream)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerMountLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.mount"))]
    pub async fn mount(&self, parameter: MountContainerParameter) -> Result<String> {
        let endpoint = format!(
            "{}{}/mount",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, None)
            .await?;
        match utils::check_service_response_for_error(service_response.clone()) {
            Ok(_) => Ok(String::from_utf8(service_response.body.to_vec())?),
            Err(e) => Err(e),
        }
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerPauseLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.pause"))]
    pub async fn pause(&self, parameter: PauseContainerParameter) -> Result<()> {
        let endpoint = format!(
            "{}{}/pause",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerRenameLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.rename"))]
    pub async fn rename(&self, parameter: RenameContainerParameter) -> Result<()> {
        let endpoint = format!(
            "{}{}/rename",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, Some(query), None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerRestartLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.restart"))]
    pub async fn restart(&self, parameter: RestartContainerParameter) -> Result<()> {
        let endpoint = format!(
            "{}{}/restart",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, Some(query), None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerTopLibpod>
    #[cfg_attr(
        feature = "enable-tracing",
        instrument(name = "Containers.list_processes")
    )]
    pub async fn list_processes(
        &self,
        parameter: ListContainerProcessesParameter,
    ) -> Result<ListContainerProcessesResponse> {
        let endpoint = format!(
            "{}{}/top",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );

        let parameter = ListContainerProcessesParameter {
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

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerTopLibpod>
    #[cfg_attr(
        feature = "enable-tracing",
        instrument(name = "Containers.list_processes_streaming")
    )]
    pub async fn list_processes_streaming(
        &self,
        parameter: ListContainerProcessesParameter,
    ) -> Result<impl Stream<Item = Result<ListContainerProcessesResponse>>> {
        let endpoint = format!(
            "{}{}/top",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let parameter = ListContainerProcessesParameter {
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

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerUnmountLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.unmount"))]
    pub async fn unmount(&self, parameter: UnmountContainerParameter) -> Result<()> {
        let endpoint = format!(
            "{}{}/unmount",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerUnpauseLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.unpause"))]
    pub async fn unpause(&self, parameter: UnpauseContainerParameter) -> Result<()> {
        let endpoint = format!(
            "{}{}/unpause",
            utils::create_endpoint("/libpod/containers/"),
            parameter.container_name
        );
        let service_response = self
            .podman_service
            .post_request(&endpoint, None, None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainersStatsAllLibpod>
    #[cfg_attr(
        feature = "enable-tracing",
        instrument(name = "Containers.stats_streaming")
    )]
    pub async fn stats_stream(
        &self,
        parameter: ContainersStatsParameter,
    ) -> Result<impl Stream<Item = Result<ContainerStatsResponse>>> {
        let endpoint = format!("{}stats", utils::create_endpoint("/libpod/containers/"));
        let parameter = ContainersStatsParameter {
            stream: Some(true),
            ..parameter
        };
        // Start workaround cause podman only supports query arrays in this format:
        // containers=container1&containers=container2
        let query = {
            use serde::Serialize;
            use serde_with::skip_serializing_none;

            #[skip_serializing_none]
            #[derive(Serialize, Debug)]
            struct TempQuery {
                interval: Option<i32>,
                stream: bool,
            }
            let temp_query = TempQuery {
                interval: parameter.interval,
                stream: true,
            };
            let temp_query = serde_qs::to_string(&temp_query)?;
            if parameter.container_names.is_empty() {
                temp_query
            } else {
                #[skip_serializing_none]
                #[derive(Serialize, Debug)]
                struct TempContainer {
                    containers: String,
                }
                let mut container_string = String::new();
                for container in parameter.container_names {
                    let temp_container = TempContainer {
                        containers: container,
                    };
                    container_string.push_str(&serde_qs::to_string(&temp_container)?);
                    container_string.push('&');
                }
                container_string.push_str(&temp_query);
                container_string
            }
        };
        // end workaround

        let (_status_code, result_stream, _header_map) = self
            .podman_service
            .get_json_stream(&endpoint, Some(query), None, None)
            .await?;
        Ok(result_stream)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainersStatsAllLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.stats"))]
    pub async fn stats(
        &self,
        parameter: ContainersStatsParameter,
    ) -> Result<ContainerStatsResponse> {
        let endpoint = format!("{}stats", utils::create_endpoint("/libpod/containers/"));
        let parameter = ContainersStatsParameter {
            stream: Some(false),
            ..parameter
        };

        // Start workaround cause podman only supports query arrays in this format:
        // containers=container1&containers=container2
        let query = {
            use serde::Serialize;
            use serde_with::skip_serializing_none;

            #[skip_serializing_none]
            #[derive(Serialize, Debug)]
            struct TempQuery {
                interval: Option<i32>,
                stream: bool,
            }
            let temp_query = TempQuery {
                interval: parameter.interval,
                stream: false,
            };
            let temp_query = serde_qs::to_string(&temp_query)?;
            if parameter.container_names.is_empty() {
                temp_query
            } else {
                #[skip_serializing_none]
                #[derive(Serialize, Debug)]
                struct TempContainer {
                    containers: String,
                }
                let mut container_string = String::new();
                for container in parameter.container_names {
                    let temp_container = TempContainer {
                        containers: container,
                    };
                    container_string.push_str(&serde_qs::to_string(&temp_container)?);
                    container_string.push('&');
                }
                container_string.push_str(&temp_query);
                container_string
            }
        };
        //end workaround

        let service_response = self
            .podman_service
            .get_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ContainerPruneLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Containers.prune"))]
    pub async fn prune(
        &self,
        parameter: PruneContainersParameter,
    ) -> Result<Vec<PruneContainerResponseEntry>> {
        let parameter: PruneContainersParameterQuery = parameter.try_into()?;
        let endpoint = utils::create_endpoint("/libpod/containers/prune");

        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }
}
