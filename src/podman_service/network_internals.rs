use crate::error::{PodtenderError, Result};
use crate::podman_service::podman_service::PodmanService;
use crate::utils;
use asynchronous_codec::{FramedRead, JsonCodec, LinesCodec};
use futures::{Stream, TryStreamExt};
use hyper::body::Bytes;
use hyper::{Body, Method, Request, StatusCode};
use hyper::{HeaderMap, Uri as HyperUri};
use hyperlocal::Uri;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
#[cfg(feature = "enable-tracing")]
use tracing::debug;

#[derive(Debug)]
pub(crate) struct PodmanServiceResponse {
    pub status_code: StatusCode,
    pub body: Body,
    pub headers: HeaderMap,
}

impl PodmanServiceResponse {
    pub(crate) fn new(status_code: StatusCode, body: Body, headers: HeaderMap) -> Self {
        PodmanServiceResponse {
            status_code,
            body,
            headers,
        }
    }
}

// allow dead code since `headers` is never read but should be kept for possible future development.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct PodmanServiceResponseBytes {
    pub status_code: StatusCode,
    pub body: Bytes,
    pub headers: HeaderMap,
}

impl PodmanServiceResponseBytes {
    pub(crate) fn new(status_code: StatusCode, body: Bytes, headers: HeaderMap) -> Self {
        PodmanServiceResponseBytes {
            status_code,
            body,
            headers,
        }
    }
}

impl PodmanService {
    /// Send a get request to the podman api.
    pub(crate) async fn get_request(
        &self,
        endpoint: &str,
        query: Option<String>,
        header: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<PodmanServiceResponseBytes> {
        let response = self
            .send_json_request(Method::GET, endpoint, query, header, body)
            .await?;
        Ok(PodmanServiceResponseBytes::new(
            response.status_code,
            hyper::body::to_bytes(response.body).await?,
            response.headers,
        ))
    }

    /// Send a post request to the podman api.
    pub(crate) async fn post_request(
        &self,
        endpoint: &str,
        query: Option<String>,
        header: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<PodmanServiceResponseBytes> {
        let response = self
            .send_json_request(Method::POST, endpoint, query, header, body)
            .await?;
        Ok(PodmanServiceResponseBytes::new(
            response.status_code,
            hyper::body::to_bytes(response.body).await?,
            response.headers,
        ))
    }

    /// Send a put request to the podman api.
    #[allow(dead_code)]
    pub(crate) async fn put_request(
        &self,
        endpoint: &str,
        query: Option<String>,
        header: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<PodmanServiceResponseBytes> {
        let response = self
            .send_json_request(Method::PUT, endpoint, query, header, body)
            .await?;
        Ok(PodmanServiceResponseBytes::new(
            response.status_code,
            hyper::body::to_bytes(response.body).await?,
            response.headers,
        ))
    }

    /// Send a delete request to the podman api.
    pub(crate) async fn delete_request(
        &self,
        endpoint: &str,
        query: Option<String>,
        header: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<PodmanServiceResponseBytes> {
        let response = self
            .send_json_request(Method::DELETE, endpoint, query, header, body)
            .await?;
        Ok(PodmanServiceResponseBytes::new(
            response.status_code,
            hyper::body::to_bytes(response.body).await?,
            response.headers,
        ))
    }

    /// Sends a get request to the podman api and returns a stream of results.
    pub(crate) async fn get_json_stream<T>(
        &self,
        endpoint: &str,
        query: Option<String>,
        header: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<(StatusCode, impl Stream<Item = Result<T>>, HeaderMap)>
    where
        T: Serialize + DeserializeOwned + 'static,
    {
        let (status_code, stream, header_map) = self
            .receive_bytes_stream(Method::GET, endpoint, query, header, body)
            .await?;
        let stream =
            Box::pin(stream.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)))
                .into_async_read();

        let codec = JsonCodec::<T, T>::new();
        Ok((
            status_code,
            FramedRead::new(stream, codec).map_err(|e| e.into()),
            header_map,
        ))
    }

    /// Sends a post request to the podman api and returns a stream of results.
    pub(crate) async fn post_json_stream<T>(
        &self,
        endpoint: &str,
        query: Option<String>,
        header: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<(StatusCode, impl Stream<Item = Result<T>>, HeaderMap)>
    where
        T: Serialize + DeserializeOwned + 'static,
    {
        let (status_code, stream, header_map) = self
            .receive_bytes_stream(Method::POST, endpoint, query, header, body)
            .await?;
        let stream =
            Box::pin(stream.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)))
                .into_async_read();

        let codec = JsonCodec::<T, T>::new();
        Ok((
            status_code,
            FramedRead::new(stream, codec).map_err(|e| e.into()),
            header_map,
        ))
    }

    /// Sends a get request to the podman api and returns a stream of lines.
    pub(crate) async fn get_lines_stream(
        &self,
        endpoint: &str,
        query: Option<String>,
        header: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<(StatusCode, impl Stream<Item = Result<String>>, HeaderMap)> {
        let (status_code, stream, header_map) = self
            .receive_bytes_stream(Method::GET, endpoint, query, header, body)
            .await?;
        let stream =
            Box::pin(stream.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)))
                .into_async_read();
        Ok((
            status_code,
            FramedRead::new(stream, LinesCodec).map_err(|e| e.into()),
            header_map,
        ))
    }

    /// Sends a get request to the podman api and returns a stream of file chunks.
    /// Used to download exports, etc.
    pub(crate) async fn get_receive_file_chunks_stream(
        &self,
        endpoint: &str,
        query: Option<String>,
        header: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<(StatusCode, impl Stream<Item = Result<Vec<u8>>>, HeaderMap)> {
        let (status_code, stream, header_map) = self
            .receive_bytes_stream(Method::GET, endpoint, query, header, body)
            .await?;
        Ok((status_code, stream.map_ok(|b| b.to_vec()), header_map))
    }

    /// Sends a post request to the podman api and returns a stream of file chunks.
    /// Used to download exports, etc.
    pub(crate) async fn post_receive_file_chunks_stream(
        &self,
        endpoint: &str,
        query: Option<String>,
        header: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<(StatusCode, impl Stream<Item = Result<Vec<u8>>>, HeaderMap)> {
        let (status_code, stream, header_map) = self
            .receive_bytes_stream(Method::POST, endpoint, query, header, body)
            .await?;
        Ok((status_code, stream.map_ok(|b| b.to_vec()), header_map))
    }

    /// Upload a file via post request.
    pub(crate) async fn post_send_file_chunks_stream<S, O, E>(
        &self,
        endpoint: &str,
        query: Option<String>,
        header: Option<HashMap<String, String>>,
        body: Option<S>,
    ) -> Result<PodmanServiceResponseBytes>
    where
        S: futures::Stream<Item = std::result::Result<O, E>> + Send + 'static,
        O: Into<Bytes> + 'static,
        E: Into<PodtenderError> + Send + 'static,
    {
        let response = self
            .send_file_request(Method::POST, endpoint, query, header, body)
            .await?;
        Ok(PodmanServiceResponseBytes::new(
            response.status_code,
            hyper::body::to_bytes(response.body).await?,
            response.headers,
        ))
    }

    /// Internal method to receive a byte stream after a request.
    async fn receive_bytes_stream(
        &self,
        method: Method,
        endpoint: &str,
        query: Option<String>,
        header: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<(StatusCode, impl Stream<Item = Result<Bytes>>, HeaderMap)> {
        let response = self
            .send_json_request(method, endpoint, query, header, body)
            .await
            .unwrap();

        if response.status_code.is_success() {
            Ok((
                response.status_code,
                response.body.map_err(|e| e.into()).into_stream(),
                response.headers,
            ))
        } else {
            let response = PodmanServiceResponseBytes::new(
                response.status_code,
                hyper::body::to_bytes(response.body).await?,
                response.headers,
            );
            let error = utils::handle_service_response_error(response);
            Err(error)
        }
    }

    /// Sends a request to the podman api.
    async fn send_json_request(
        &self,
        method: Method,
        endpoint: &str,
        query: Option<String>,
        header: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<PodmanServiceResponse> {
        let endpoint = if let Some(query) = query {
            format!("{}?{}", endpoint, query)
        } else {
            String::from(endpoint)
        };

        let mut request_builder = Request::builder()
            .method(method)
            .uri(Into::<HyperUri>::into(Uri::new(
                &self.path,
                endpoint.as_str(),
            )));
        if let Some(header) = header {
            for (key, value) in header {
                request_builder = request_builder.header(key.as_str(), value.as_str());
            }
        };
        let request = if let Some(body) = body {
            request_builder = request_builder.header("content-type", "application/json");
            request_builder.body(Body::from(body))?
        } else {
            request_builder.body(Body::empty())?
        };

        #[cfg(feature = "enable-tracing")]
        debug!(?request);

        let response = self.client.request(request).await?;

        #[cfg(feature = "enable-tracing")]
        debug!(?response);

        let (parts, body) = response.into_parts();
        Ok(PodmanServiceResponse::new(
            parts.status,
            body,
            parts.headers,
        ))
    }

    /// Upload a file in the body as `application/x-tar`.
    async fn send_file_request<S, O, E>(
        &self,
        method: Method,
        endpoint: &str,
        query: Option<String>,
        header: Option<HashMap<String, String>>,
        body: Option<S>,
    ) -> Result<PodmanServiceResponse>
    where
        S: futures::Stream<Item = std::result::Result<O, E>> + Send + 'static,
        O: Into<Bytes> + 'static,
        E: Into<PodtenderError> + Send + 'static,
    {
        let endpoint = if let Some(query) = query {
            format!("{}?{}", endpoint, query)
        } else {
            String::from(endpoint)
        };

        let mut request_builder = Request::builder()
            .method(method)
            .uri(Into::<HyperUri>::into(Uri::new(
                &self.path,
                endpoint.as_str(),
            )));
        if let Some(header) = header {
            for (key, value) in header {
                request_builder = request_builder.header(key.as_str(), value.as_str());
            }
        };
        let request = if let Some(body) = body {
            request_builder = request_builder.header("content-type", "application/x-tar");
            let body = body.map_err(Into::into);
            request_builder.body(Body::wrap_stream(body))?
        } else {
            request_builder.body(Body::empty())?
        };

        #[cfg(feature = "enable-tracing")]
        debug!(?request);

        let response = self.client.request(request).await?;

        #[cfg(feature = "enable-tracing")]
        debug!(?response);

        let (parts, body) = response.into_parts();
        Ok(PodmanServiceResponse::new(
            parts.status,
            body,
            parts.headers,
        ))
    }
}
