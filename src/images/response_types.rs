use crate::containers::parameter_types::Schema2HealthConfig;
use crate::containers::response_types::DriverData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RemoveImageResponse {
    #[serde(rename = "Untagged")]
    pub untagged: Option<Vec<String>>,
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<u32>,
    #[serde(rename = "Errors")]
    pub errors: Option<Vec<String>>,
    #[serde(rename = "Deleted")]
    pub deleted: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectImageResponse {
    #[serde(rename = "Annotations")]
    pub annotations: Option<HashMap<String, String>>,
    #[serde(rename = "Architecture")]
    pub architecture: Option<String>,
    #[serde(rename = "Author")]
    pub author: Option<String>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "Config")]
    pub config: Option<ImageConfig>,
    #[serde(rename = "Created")]
    pub created: Option<String>,
    #[serde(rename = "Digest")]
    pub digest: Option<String>,
    #[serde(rename = "GraphDriver")]
    pub graph_driver: Option<DriverData>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Option<Schema2HealthConfig>,
    #[serde(rename = "History")]
    pub history: Option<Vec<ImageLayer>>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "ManifestType")]
    pub manifest_type: Option<String>,
    #[serde(rename = "NamesHistory")]
    pub names_history: Option<Vec<String>>,
    #[serde(rename = "Os")]
    pub os: Option<String>,
    #[serde(rename = "Parent")]
    pub parent: Option<String>,
    #[serde(rename = "RepoDigests")]
    pub repo_digests: Option<Vec<String>>,
    #[serde(rename = "RepoTags")]
    pub repo_tags: Option<Vec<String>>,
    #[serde(rename = "RootFS")]
    pub root_fs: Option<RootFs>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    #[serde(rename = "User")]
    pub user: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "VirtualSize")]
    pub virtual_size: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImageConfig {
    #[serde(rename = "Cmd")]
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Option<Vec<String>>,
    #[serde(rename = "Env")]
    pub env: Option<Vec<String>>,
    #[serde(rename = "ExposedPorts")]
    pub exposed_ports: Option<HashMap<String, String>>,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "StopSignal")]
    pub stop_signal: Option<String>,
    #[serde(rename = "User")]
    pub user: Option<String>,
    #[serde(rename = "Volumes")]
    pub volumes: Option<HashMap<String, String>>,
    #[serde(rename = "WorkingDir")]
    pub working_dir: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImageLayer {
    pub author: Option<Vec<String>>,
    pub comment: Option<String>,
    pub created: Option<String>,
    pub created_by: Option<String>,
    pub empty_layer: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RootFs {
    #[serde(rename = "Layers")]
    pub layers: Option<Vec<String>>,
    #[serde(rename = "Type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImportImageResponse {
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ListImagesResponseEntry {
    #[serde(rename = "ConfigDigest")]
    pub config_digest: Option<String>,
    #[serde(rename = "Containers")]
    pub containers: Option<i64>,
    #[serde(rename = "Created")]
    pub created: Option<i64>,
    #[serde(rename = "Dangling")]
    pub dangling: Option<String>,
    #[serde(rename = "Digest")]
    pub digest: Option<String>,
    #[serde(rename = "History")]
    pub history: Option<Vec<String>>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Names")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "ParentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,
    #[serde(rename = "RepoDigests")]
    pub repo_digests: Option<Vec<String>>,
    #[serde(rename = "RepoTags")]
    pub repo_tags: Option<Vec<String>>,
    #[serde(rename = "SharedSize")]
    pub shared_size: Option<i64>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    #[serde(rename = "VirtualSize")]
    pub virtual_size: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LoadImageResponse {
    #[serde(rename = "Names")]
    pub names: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PruneImagesResponseEntry {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Err")]
    pub err: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PullImagesResponse {
    pub error: Option<String>,
    pub id: Option<String>,
    pub images: Option<Vec<String>>,
    pub stream: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SearchImagesResponseEntry {
    #[serde(rename = "Automated")]
    pub automated: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Index")]
    pub index: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Official")]
    pub official: Option<String>,
    #[serde(rename = "Stars")]
    pub stars: Option<i64>,
    #[serde(rename = "Tag")]
    pub tag: Option<String>,
}
