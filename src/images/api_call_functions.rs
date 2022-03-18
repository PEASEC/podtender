use crate::error::{PodtenderError, Result};
use crate::images::parameter_types::*;
use crate::images::response_types::*;
use crate::podman_service::PodmanService;
use crate::utils;
use futures::Stream;
use hyper::body::Bytes;
use std::collections::HashMap;
use std::convert::TryInto;
#[cfg(feature = "enable-tracing")]
use tracing::instrument;

/// Container operations.
#[derive(Debug)]
pub struct Images<'service> {
    podman_service: &'service PodmanService,
}

impl<'service> Images<'service> {
    pub(crate) fn new(podman_service: &'service PodmanService) -> Self {
        Images { podman_service }
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ImageDeleteLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Images.remove"))]
    pub async fn remove(&self, parameter: RemoveImageParameter) -> Result<RemoveImageResponse> {
        let endpoint = utils::create_endpoint(&format!("/libpod/images/{}", parameter.image_name));
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .delete_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ImageExistsLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Images.exists"))]
    pub async fn exists(&self, parameter: ImageExistsParameter) -> Result<()> {
        let endpoint =
            utils::create_endpoint(&format!("/libpod/images/{}/exists", parameter.image_name));
        let service_response = self
            .podman_service
            .get_request(&endpoint, None, None, None)
            .await?;
        utils::check_service_response_for_error(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ImageGetLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Images.export"))]
    pub async fn export(
        &self,
        parameter: ExportImageParameter,
    ) -> Result<impl Stream<Item = Result<Vec<u8>>>> {
        let endpoint = format!(
            "{}{}/get",
            utils::create_endpoint("/libpod/images/"),
            parameter.image_name
        );

        let (_status_code, result_stream, _header_map) = self
            .podman_service
            .get_receive_file_chunks_stream(&endpoint, None, None, None)
            .await?;
        Ok(result_stream)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ImageInspectLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Images.inspect"))]
    pub async fn inspect(&self, parameter: InspectImageParameter) -> Result<InspectImageResponse> {
        let endpoint = format!(
            "{}{}/json",
            utils::create_endpoint("/libpod/images/"),
            parameter.image_name
        );
        let service_response = self
            .podman_service
            .get_request(&endpoint, None, None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ImageListLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Images.list"))]
    pub async fn list(
        &self,
        parameter: ListImagesParameter,
    ) -> Result<Vec<ListImagesResponseEntry>> {
        let endpoint = utils::create_endpoint("/libpod/images/json");
        let parameter: ListImagesParameterQuery = parameter.try_into()?;
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .get_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ImagePruneLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Images.prune"))]
    pub async fn prune(
        &self,
        parameter: PruneImagesParameter,
    ) -> Result<Vec<PruneImagesResponseEntry>> {
        let endpoint = utils::create_endpoint("/libpod/images/prune");
        let parameter: PruneImagesParameterQuery = parameter.try_into()?;
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ImagePullLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Images.pull"))]
    pub async fn pull(
        &self,
        parameter: PullImagesParameter,
    ) -> Result<impl Stream<Item = Result<PullImagesResponse>>> {
        let endpoint = utils::create_endpoint("/libpod/images/pull");

        let header = {
            if parameter.x_registry_auth_header.is_some() {
                let mut header_map = HashMap::new();
                header_map.insert(
                    String::from("X-Registry-Auth"),
                    parameter.x_registry_auth_header.as_ref().unwrap().clone(),
                );
                Some(header_map)
            } else {
                None
            }
        };
        let query = serde_qs::to_string(&parameter)?;

        let (_status_code, result_stream, _header_map) = self
            .podman_service
            .post_json_stream(&endpoint, Some(query), header, None)
            .await?;
        Ok(result_stream)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ImageSearchLibpod>
    #[cfg_attr(feature = "enable-tracing", instrument(name = "Images.search"))]
    pub async fn search(
        &self,
        parameter: SearchImagesParameter,
    ) -> Result<Vec<SearchImagesResponseEntry>> {
        let endpoint = utils::create_endpoint("/libpod/images/search");
        let parameter: SearchImagesParameterQuery = parameter.try_into()?;
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .get_request(&endpoint, Some(query), None, None)
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ImageImportLibpod>
    #[cfg_attr(
        feature = "enable-tracing",
        instrument(name = "Images.import", skip(file_stream))
    )]
    pub async fn import<S, O, E>(
        &self,
        parameter: ImportImageParameter,
        file_stream: S,
    ) -> Result<ImportImageResponse>
    where
        S: futures::Stream<Item = std::result::Result<O, E>> + Send + 'static,
        O: Into<Bytes> + 'static,
        E: Into<PodtenderError> + Send + 'static,
    {
        let endpoint = utils::create_endpoint("/libpod/images/import");
        let query = serde_qs::to_string(&parameter)?;
        let service_response = self
            .podman_service
            .post_send_file_chunks_stream(&endpoint, Some(query), None, Some(file_stream))
            .await?;
        utils::deserialize_service_response(service_response)
    }

    /// <https://docs.podman.io/en/latest/_static/api.html?version=v4.0#operation/ImageLoadLibpod>
    #[cfg_attr(
        feature = "enable-tracing",
        instrument(name = "Images.load", skip(file_stream))
    )]
    pub async fn load<S, O, E>(&self, file_stream: S) -> Result<LoadImageResponse>
    where
        S: futures::Stream<Item = std::result::Result<O, E>> + Send + 'static,
        O: Into<Bytes> + 'static,
        E: Into<PodtenderError> + Send + 'static,
    {
        let endpoint = utils::create_endpoint("/libpod/images/load");

        let service_response = self
            .podman_service
            .post_send_file_chunks_stream(&endpoint, None, None, Some(file_stream))
            .await?;
        utils::deserialize_service_response(service_response)
    }
}
