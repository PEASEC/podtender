use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type CreateVolumeResponse = InspectVolumeResponse;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectVolumeResponse {
    #[serde(rename = "Anonymous")]
    pub anonymous: Option<bool>,
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<String>,
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "GID")]
    pub gid: Option<i64>,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "MountCount")]
    pub mound_count: Option<u64>,
    #[serde(rename = "Mountpoint")]
    pub mountpoint: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NeedsChown")]
    pub needs_chown: Option<bool>,
    #[serde(rename = "NeedsCopyUp")]
    pub needs_copy_up: Option<bool>,
    #[serde(rename = "Options")]
    pub options: Option<HashMap<String, String>>,
    #[serde(rename = "Scope")]
    pub scope: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<HashMap<String, String>>,
    #[serde(rename = "UID")]
    pub uid: Option<i64>,
}

pub type ListVolumesResponseEntry = InspectVolumeResponse;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VolumeUsageData {
    #[serde(rename = "RefCount")]
    pub ref_count: Option<i64>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ErrIdSizeResponse {
    #[serde(rename = "Err")]
    pub err: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<u64>,
}
pub type PruneVolumesResponseEntry = ErrIdSizeResponse;
