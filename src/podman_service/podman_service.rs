use crate::containers::Containers;
use crate::images::Images;
use crate::networks::Networks;
use crate::pods::Pods;
use crate::system::System;
use crate::volumes::Volumes;
use hyper::Client;
use hyperlocal::{UnixClientExt, UnixConnector};
use std::path::PathBuf;

/// The podman service. Only a unix socket is supported.
#[derive(Debug, Clone)]
pub struct PodmanService {
    pub(crate) path: String,
    pub(crate) client: Client<UnixConnector>,
}

/// `path` expects a path to the podman socket as `&str`.
impl PodmanService {
    pub fn new(path: &str) -> Self {
        PodmanService {
            path: path.to_string(),
            client: Client::unix(),
        }
    }

    /// Check whether the podman socket exists.
    pub fn check_socket_exists(&self) -> bool {
        PathBuf::from(&self.path).exists()
    }

    pub fn system(&self) -> System {
        System::new(self)
    }

    pub fn containers(&self) -> Containers {
        Containers::new(self)
    }

    pub fn volumes(&self) -> Volumes {
        Volumes::new(self)
    }

    pub fn networks(&self) -> Networks {
        Networks::new(self)
    }

    pub fn pods(&self) -> Pods {
        Pods::new(self)
    }

    pub fn images(&self) -> Images {
        Images::new(self)
    }
}
