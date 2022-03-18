use crate::containers::parameter_types::{
    ImageVolume, LinuxResources, LinuxThrottleDevice, Mount, NamedVolume, Namespace, OverlayVolume,
    PerNetworkOptions, PortMapping,
};
use crate::error::PodtenderError;
#[cfg(any(test, feature = "examples"))]
use crate::example_values_trait::ExampleValues;
use crate::utils;
#[cfg(feature = "builder")]
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::convert::TryFrom;

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct CreatePodParameter {
    pub cgroup_parent: Option<String>,
    pub cpu_period: Option<i64>,
    pub cpu_quota: Option<i64>,
    pub dns_option: Option<Vec<String>>,
    pub dns_search: Option<Vec<String>>,
    pub dns_server: Option<String>,
    pub hostadd: Option<Vec<String>>,
    pub hostname: Option<String>,
    pub image_volumes: Option<ImageVolume>,
    pub infra_command: Option<Vec<String>>,
    pub infra_conmon_pid_file: Option<String>,
    pub infra_image: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub mounts: Option<Vec<Mount>>,
    pub name: Option<String>,
    pub netns: Option<Namespace>,
    pub network_options: Option<HashMap<String, String>>,
    #[serde(rename = "Networks")]
    pub networks: Option<HashMap<String, PerNetworkOptions>>,
    pub no_infra: Option<bool>,
    pub no_manage_hosts: Option<bool>,
    pub no_manage_resolv_conf: Option<bool>,
    pub overlay_volumes: Option<Vec<OverlayVolume>>,
    pub pidns: Option<Namespace>,
    pub pod_create_command: Option<String>,
    pub pod_devices: Option<Vec<String>>,
    pub portmappings: Option<Vec<PortMapping>>,
    pub resource_limits: Option<LinuxResources>,
    pub security_opt: Option<Vec<String>>,
    pub shared_namespaces: Option<Vec<String>>,
    pub sysctl: Option<HashMap<String, String>>,
    #[serde(rename = "ThrottleReadBpsDevice")]
    pub throttle_read_bps_device: Option<HashMap<String, LinuxThrottleDevice>>,
    pub userns: Option<Namespace>,
    pub volumes: Option<Vec<NamedVolume>>,
    pub volumes_from: Option<Vec<String>>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for CreatePodParameter {
    fn example() -> Self {
        Self {
            name: Some(String::from("CreatePodParameter")),
            ..Default::default()
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct RemovePodParameter {
    #[serde(skip_serializing)]
    pub pod_name: String,
    pub force: Option<bool>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for RemovePodParameter {
    fn example() -> Self {
        Self {
            pod_name: String::from("RemovePodParameter"),
            force: Some(true),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct PodExistsParameter {
    pub pod_name: String,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for PodExistsParameter {
    fn example() -> Self {
        Self {
            pod_name: String::from("ExistsPodParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct InspectPodParameter {
    pub pod_name: String,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for InspectPodParameter {
    fn example() -> Self {
        Self {
            pod_name: String::from("InspectPodParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct KillPodParameter {
    #[serde(skip_serializing)]
    pub pod_name: String,
    pub signal: Option<String>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for KillPodParameter {
    fn example() -> Self {
        Self {
            pod_name: String::from("KillPodParameter"),
            signal: Some(String::from("SIGKILL")),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct PausePodParameter {
    #[serde(skip_serializing)]
    pub pod_name: String,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for PausePodParameter {
    fn example() -> Self {
        Self {
            pod_name: String::from("PausePodParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct RestartPodParameter {
    #[serde(skip_serializing)]
    pub pod_name: String,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for RestartPodParameter {
    fn example() -> Self {
        Self {
            pod_name: String::from("RestartPodParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct StartPodParameter {
    #[serde(skip_serializing)]
    pub pod_name: String,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for StartPodParameter {
    fn example() -> Self {
        Self {
            pod_name: String::from("StartPodParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct StopPodParameter {
    #[serde(skip_serializing)]
    pub pod_name: String,
    #[serde(rename = "t")]
    pub timeout: Option<i32>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for StopPodParameter {
    fn example() -> Self {
        Self {
            pod_name: String::from("StopPodParameter"),
            timeout: Some(0),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ListPodProcessesParameter {
    #[serde(skip_serializing)]
    pub pod_name: String,
    pub delay: Option<u32>,
    pub ps_args: Option<String>,
    pub stream: Option<bool>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ListPodProcessesParameter {
    fn example() -> Self {
        Self {
            pod_name: String::from("PodListProcessesParameter"),
            delay: Some(1),
            ps_args: None,
            stream: Some(true),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct UnpausePodParameter {
    #[serde(skip_serializing)]
    pub pod_name: String,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for UnpausePodParameter {
    fn example() -> Self {
        Self {
            pod_name: String::from("UnpausePodParameter"),
        }
    }
}

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ListPodsParameter {
    pub filters: Option<HashMap<String, Vec<String>>>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ListPodsParameter {
    fn example() -> Self {
        let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
        filter_map.insert(
            String::from("label"),
            vec![String::from("ListPodsParameter")],
        );
        Self {
            filters: Some(filter_map),
        }
    }
}

/// Internal representation of `ListPodsParameter` since filters can't be serialized in a single step.
/// It needs to be serialized to json, then to query. `TryInto` tries to perform the serialisation into json.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct ListPodsParameterQuery {
    pub filters: Option<String>,
}

impl TryFrom<ListPodsParameter> for ListPodsParameterQuery {
    type Error = PodtenderError;
    fn try_from(param: ListPodsParameter) -> Result<Self, Self::Error> {
        let filters = utils::convert_from_map_to_json_string(param.filters)?;
        Ok(ListPodsParameterQuery { filters })
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct PodStatsParameter {
    pub all: Option<bool>,
    #[serde(rename = "namesOrIDs")]
    pub names_or_ids: Option<Vec<String>>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for PodStatsParameter {
    fn example() -> Self {
        Self {
            all: Some(true),
            names_or_ids: None,
        }
    }
}
