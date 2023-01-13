use crate::error::PodtenderError;
#[cfg(any(test, feature = "examples"))]
use crate::example_values_trait::ExampleValues;
use crate::system::response_types::IdMap;
use crate::utils;
#[cfg(feature = "builder")]
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::convert::TryFrom;

//json
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct CreateContainerParameter {
    pub annotations: Option<HashMap<String, String>>,
    pub apparmor_profile: Option<String>,
    pub cap_add: Option<String>,
    pub cap_drop: Option<Vec<String>>,
    pub cgroup_parent: Option<String>,
    pub cgroupns: Option<Namespace>,
    pub cgroups_mode: Option<String>,
    pub command: Option<Vec<String>>,
    pub conmon_pid_file: Option<String>,
    #[serde(rename = "containerCreateCommand")]
    pub container_create_command: Option<Vec<String>>,
    pub cpu_period: Option<u64>,
    pub cpu_quota: Option<u64>,
    pub create_working_dir: Option<bool>,
    #[serde(rename = "dependencyContainers")]
    pub dependency_containers: Option<Vec<String>>,
    pub device_cgroup_rule: Option<Vec<LinuxDeviceCgroup>>,
    pub devices: Option<Vec<LinuxDevice>>,
    pub devices_from: Option<Vec<String>>,
    pub dns_option: Option<Vec<String>>,
    pub dns_search: Option<Vec<String>>,
    pub dns_server: Option<Vec<Vec<u32>>>,
    pub entrypoint: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
    pub env_host: Option<bool>,
    pub expose: Option<HashMap<u16, String>>,
    pub groups: Option<Vec<String>>,
    pub healthconfig: Option<Schema2HealthConfig>,
    pub host_device_list: Option<Vec<LinuxDevice>>,
    pub hostadd: Option<Vec<String>>,
    pub hostname: Option<String>,
    pub hostusers: Option<Vec<String>>,
    pub httpproxy: Option<bool>,
    pub idmappings: Option<IdMappingOptions>,
    pub image: Option<String>,
    pub image_volume_mode: Option<String>,
    pub image_volumes: Option<Vec<ImageVolume>>,
    pub init: Option<bool>,
    pub init_container_type: Option<String>,
    pub init_path: Option<String>,
    pub ipcns: Option<Namespace>,
    pub labels: Option<HashMap<String, String>>,
    pub log_configuration: Option<LogConfig>,
    pub manage_password: Option<bool>,
    pub mask: Option<Vec<String>>,
    pub mounts: Option<Vec<Mount>>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub netns: Option<Namespace>,
    pub network_options: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "Networks")]
    pub networks: Option<HashMap<String, PerNetworkOptions>>,
    pub no_new_privileges: Option<bool>,
    pub oci_runtime: Option<String>,
    pub oom_score_adj: Option<i64>,
    pub overlay_columes: Option<Vec<OverlayVolume>>,
    pub personality: Option<LinuxPersonality>,
    pub pidns: Option<Namespace>,
    pub pod: Option<String>,
    pub portmappings: Option<Vec<PortMapping>>,
    pub privileged: Option<bool>,
    pub procfs_opts: Option<Vec<String>>,
    pub publish_image_ports: Option<bool>,
    pub r_limits: Option<Vec<POSIXRlimit>>,
    pub raw_image_name: Option<String>,
    pub read_only_filesystem: Option<bool>,
    pub remove: Option<bool>,
    pub resource_limits: Option<LinuxResources>,
    pub restart_policy: Option<String>,
    pub restart_tries: Option<u64>,
    pub rootfs: Option<String>,
    pub rootfs_overlay: Option<bool>,
    pub rootfs_progagation: Option<String>,
    #[serde(rename = "sdnotifyMode")]
    pub sdnotify_mode: Option<String>,
    pub seccomp_policy: Option<String>,
    pub seccomp_profile_path: Option<String>,
    pub secrets_env: Option<HashMap<String, String>>,
    pub secrets: Option<Vec<Secret>>,
    pub selinux_opts: Option<Vec<String>>,
    pub shm_size: Option<i64>,
    pub stdin: Option<bool>,
    pub stop_signal: Option<i64>,
    pub stop_timeout: Option<u64>,
    pub storage_opts: Option<HashMap<String, String>>,
    pub sysctl: Option<HashMap<String, String>>,
    pub systemd: Option<String>,
    pub terminal: Option<bool>,
    #[serde(rename = "throttleReadBpsDevice")]
    pub throttle_read_bps_device: Option<LinuxThrottleDevice>,
    #[serde(rename = "throttleReadIOPSDevice")]
    pub throttle_read_iops_device: Option<LinuxThrottleDevice>,
    #[serde(rename = "throttleWriteBpsDevice")]
    pub throttle_write_bps_device: Option<LinuxThrottleDevice>,
    #[serde(rename = "throttleWriteIOPSDevice")]
    pub throttle_write_iops_device: Option<LinuxThrottleDevice>,
    pub timeout: Option<u64>,
    pub timezone: Option<String>,
    pub umask: Option<String>,
    pub unified: Option<HashMap<String, String>>,
    pub unmask: Option<Vec<String>>,
    pub unsetenv: Option<Vec<String>>,
    pub unsetenvall: Option<bool>,
    pub use_image_hosts: Option<bool>,
    pub use_image_resolve_conf: Option<bool>,
    pub user: Option<String>,
    pub userns: Option<Namespace>,
    pub utsns: Option<Namespace>,
    pub volatile: Option<bool>,
    pub volumes: Option<Vec<NamedVolume>>,
    pub volumes_from: Option<Vec<String>>,
    #[serde(rename = "weightDevice")]
    pub weight_device: Option<LinuxWeightDevice>,
    pub work_dir: Option<String>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for CreateContainerParameter {
    fn example() -> Self {
        Self {
            name: Some(String::from("CreateContainerParameter")),
            ..Default::default()
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct Namespace {
    pub nsmode: Option<String>,
    pub value: Option<String>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct Secret {
    #[serde(rename = "GID")]
    pub gid: Option<u32>,
    #[serde(rename = "Mode")]
    pub mode: Option<u32>,
    #[serde(rename = "Source")]
    pub source: Option<String>,
    #[serde(rename = "Target")]
    pub target: Option<String>,
    #[serde(rename = "UID")]
    pub uid: Option<u32>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxDevice {
    #[serde(rename = "fileMode")]
    pub file_mode: Option<u32>,
    pub gid: Option<u32>,
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub path: Option<String>,
    #[serde(rename = "type")]
    pub linux_device_type: Option<String>,
    pub uid: Option<u32>,
}

// also used in `InspectImageResponse`
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
#[serde(deny_unknown_fields)]
pub struct Schema2HealthConfig {
    #[serde(rename = "Interval")]
    pub interval: Option<i64>,
    #[serde(rename = "Retries")]
    pub retries: Option<i64>,
    #[serde(rename = "StartPeriod")]
    pub start_period: Option<i64>,
    #[serde(rename = "Test")]
    pub test: Option<Vec<String>>,
    #[serde(rename = "Timeout")]
    pub timeout: Option<i64>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxPersonality {
    pub domain: Option<String>,
    pub flags: Option<Vec<String>>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct IdMappingOptions {
    #[serde(rename = "AutoUserNs")]
    pub auto_user_ns: Option<bool>,
    #[serde(rename = "AutoUserNsOpts")]
    pub auto_user_ns_opts: Option<AutoUserNsOptions>,
    #[serde(rename = "GIDMap")]
    pub gid_map: Option<Vec<IdMap>>,
    #[serde(rename = "HostGidMapping")]
    pub host_gid_mapping: Option<bool>,
    #[serde(rename = "HostUIDMapping")]
    pub host_uid_mapping: Option<bool>,
    #[serde(rename = "UIDMap")]
    pub uid_map: Option<Vec<IdMap>>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct AutoUserNsOptions {
    #[serde(rename = "AdditionalGIDMappings")]
    pub additional_gid_mappings: Option<Vec<IdMap>>,
    #[serde(rename = "AdditionalUIDMappings")]
    pub additional_uid_mappings: Option<Vec<IdMap>>,
    #[serde(rename = "GroupFile")]
    pub group_file: Option<String>,
    #[serde(rename = "InitialSize")]
    pub initial_size: Option<u32>,
    #[serde(rename = "PasswdFile")]
    pub passwd_file: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<u32>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ImageVolume {
    #[serde(rename = "Destination")]
    pub destination: Option<String>,
    #[serde(rename = "ReadWrite")]
    pub read_write: Option<bool>,
    #[serde(rename = "Source")]
    pub source: Option<String>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LogConfig {
    pub driver: Option<String>,
    pub options: Option<HashMap<String, String>>,
    pub path: Option<String>,
    pub size: Option<i64>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct Mount {
    #[serde(rename = "BindOptions")]
    pub bindoptions: Option<BindOptions>,
    #[serde(rename = "Consistency")]
    pub consistency: Option<String>,
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,
    #[serde(rename = "Source")]
    pub source: Option<String>,
    #[serde(rename = "Target")]
    pub target: Option<String>,
    #[serde(rename = "TmpfsOptions")]
    pub tmpfs_options: Option<TmpfsOptions>,
    #[serde(rename = "Type")]
    pub mount_type: Option<String>,
    #[serde(rename = "VolumeOptions")]
    pub volume_options: Option<VolumeOptions>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct BindOptions {
    #[serde(rename = "NonRecursive")]
    pub non_recursive: Option<bool>,
    #[serde(rename = "Propagation")]
    pub propagation: Option<String>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct PerNetworkOptions {
    pub aliases: Option<Vec<String>>,
    pub interface_name: Option<String>,
    pub static_ips: Option<Vec<String>>,
    pub static_mac: Option<String>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct VolumeOptions {
    #[serde(rename = "DriverConfig")]
    pub driver_config: Option<DriverConfig>,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "NoCopy")]
    pub no_copy: Option<bool>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct DriverConfig {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct TmpfsOptions {
    #[serde(rename = "Mode")]
    pub mode: Option<u32>,
    #[serde(rename = "SizedBytes")]
    pub sized_bytes: Option<i64>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct OverlayVolume {
    pub destination: Option<String>,
    pub options: Option<Vec<String>>,
    pub source: Option<String>,
}

// also used in responses
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
#[serde(deny_unknown_fields)]
pub struct PortMapping {
    pub container_port: Option<u16>,
    pub host_ip: Option<String>,
    pub host_port: Option<u16>,
    pub protocol: Option<String>,
    pub range: Option<u16>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct POSIXRlimit {
    pub hard: Option<u64>,
    pub soft: Option<u64>,
    #[serde(rename = "type")]
    pub posix_rlimit_type: Option<String>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxResources {
    #[serde(rename = "blockIO")]
    pub block_io: Option<LinuxBlockIO>,
    pub cpu: Option<LinuxCPU>,
    pub devices: Option<Vec<LinuxDeviceCgroup>>,
    #[serde(rename = "hugepageLimits")]
    pub hugepage_limits: Option<Vec<LinuxHugepageLimit>>,
    pub memory: Option<LinuxMemory>,
    pub network: Option<LinuxNetwork>,
    pub pids: Option<LinuxPids>,
    pub rdma: Option<HashMap<String, LinuxRdma>>,
    pub unified: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxRdma {
    #[serde(rename = "hcaHandles")]
    pub hca_handles: Option<u32>,
    #[serde(rename = "hcaObjects")]
    pub hca_objects: Option<u32>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxBlockIO {
    #[serde(rename = "leafWeight")]
    leaf_weight: Option<u16>,
    #[serde(rename = "throttleReadBpsDevice")]
    pub throttle_read_bps_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(rename = "throttleReadIOPSDevice")]
    pub throttle_read_iops_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(rename = "throttleWriteBpsDevice")]
    pub throttle_write_bps_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(rename = "throttleWriteIOPSDevice")]
    pub throttle_write_iops_device: Option<Vec<LinuxThrottleDevice>>,
    pub weight: Option<u16>,
    #[serde(rename = "weightDevice")]
    pub weight_device: Option<Vec<LinuxWeightDevice>>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxWeightDevice {
    #[serde(rename = "leafWeight")]
    leaf_weight: Option<u16>,
    major: Option<i64>,
    minor: Option<i64>,
    weight: Option<u16>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxCPU {
    pub cpus: Option<String>,
    pub mems: Option<String>,
    pub period: Option<u64>,
    pub quote: Option<i64>,
    #[serde(rename = "realtimePeriod")]
    pub realtime_period: Option<u64>,
    #[serde(rename = "realtimeRuntime")]
    pub reatlime_runtime: Option<i64>,
    pub shares: Option<u64>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxDeviceCgroup {
    pub access: Option<String>,
    pub allow: Option<bool>,
    pub major: Option<i64>,
    pub minor: Option<i64>,
    #[serde(rename = "type")]
    pub linux_device_c_group_type: Option<String>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxHugepageLimit {
    pub limit: Option<u64>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<String>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxMemory {
    #[serde(rename = "disableOOMKiller")]
    pub disable_oom_killer: Option<bool>,
    pub kernel: Option<i64>,
    #[serde(rename = "KernelTCP")]
    pub kernel_tcp: Option<i64>,
    pub limit: Option<i64>,
    pub reservation: Option<i64>,
    pub swap: Option<i64>,
    pub swappiness: Option<u64>,
    pub use_hierarchy: Option<bool>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxNetwork {
    pub class_id: Option<u32>,
    pub priorities: Option<Vec<LinuxInterfacePriority>>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxInterfacePriority {
    pub name: Option<String>,
    pub priority: Option<u32>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxPids {
    pub limit: i64,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LinuxThrottleDevice {
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub rate: Option<i64>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct NamedVolume {
    #[serde(rename = "Dest")]
    pub dest: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<Vec<String>>,
}

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ListContainersParameter {
    pub all: Option<bool>,
    pub filters: Option<HashMap<String, Vec<String>>>,
    pub limit: Option<i32>,
    pub pod: Option<bool>,
    pub size: Option<bool>,
    pub sync: Option<bool>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ListContainersParameter {
    fn example() -> Self {
        let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
        filter_map.insert(String::from("label"), vec![String::from("list_example")]);
        Self {
            all: Some(true),
            filters: Some(filter_map),
            ..Default::default()
        }
    }
}

/// Internal representation of `ListNetworksParameter` since filters can't be serialized in a single step.
/// It needs to be serialized to json, then to query. `TryInto` tries to perform the serialisation into json.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct ListContainersParameterQuery {
    #[serde(rename = "all")]
    pub all: Option<bool>,
    #[serde(rename = "filters")]
    pub filters: Option<String>,
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    #[serde(rename = "pod")]
    pub pod: Option<bool>,
    #[serde(rename = "size")]
    pub size: Option<bool>,
    #[serde(rename = "sync")]
    pub sync: Option<bool>,
}

impl TryFrom<ListContainersParameter> for ListContainersParameterQuery {
    type Error = PodtenderError;
    fn try_from(param: ListContainersParameter) -> Result<Self, Self::Error> {
        let filters = utils::convert_from_map_to_json_string(param.filters)?;
        Ok(ListContainersParameterQuery {
            all: param.all,
            filters,
            limit: param.limit,
            pod: param.pod,
            size: param.size,
            sync: param.sync,
        })
    }
}

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct DeleteContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
    pub depend: Option<bool>,
    pub force: Option<bool>,
    pub ignore: Option<bool>,
    pub timeout: Option<u32>,
    pub v: Option<bool>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for DeleteContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("DeleteContainerParameter"),
            depend: Some(false),
            force: Some(true),
            ignore: Some(false),
            timeout: Some(0),
            v: Some(true),
        }
    }
}

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct StartContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
    #[serde(rename = "detachKeys")]
    pub detach_keys: Option<String>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for StartContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("StartContainerParameter"),
            detach_keys: Some(String::from("ctrl-p,ctrl-q")),
        }
    }
}

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct StopContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
    pub all: Option<bool>,
    #[serde(rename = "Ignore")]
    pub ignore: Option<bool>,
    pub timeout: Option<u32>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for StopContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("StopContainerParameter"),
            all: Some(false),
            ignore: Some(false),
            timeout: Some(0),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ContainerExistsParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ContainerExistsParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("ExistsContainerParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct CheckpointContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
    pub export: Option<bool>,
    #[serde(rename = "IgnoreRootFS")]
    pub ignore_root_fs: Option<bool>,
    pub keep: Option<bool>,
    #[serde(rename = "leaveRunning")]
    pub leave_running: Option<bool>,
    #[serde(rename = "printStats")]
    pub print_stats: Option<bool>,
    #[serde(rename = "tcpEstablished")]
    pub tcp_established: Option<bool>,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for CheckpointContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("CheckpointContainerParameter"),
            export: Some(true),
            ignore_root_fs: Some(false),
            keep: Some(false),
            leave_running: Some(false),
            print_stats: Some(false),
            tcp_established: Some(true),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ExportContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ExportContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("ExportContainerParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct HealthcheckContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for HealthcheckContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("HealthcheckParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct InitializeContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for InitializeContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("InitializeContainerParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct InspectContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
    pub size: Option<bool>,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for InspectContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("InspectContainerParameter"),
            size: Some(true),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct KillContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
    pub signal: Option<String>,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for KillContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("KillContainerParameter"),
            signal: Some(String::from("TERM")),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ContainerLogsParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
    pub follow: Option<bool>,
    pub since: Option<bool>,
    pub stdout: Option<bool>,
    pub stderr: Option<bool>,
    pub tail: Option<bool>,
    pub timestamps: Option<bool>,
    pub until: Option<bool>,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ContainerLogsParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("ContainerLogsParameter"),
            follow: Some(true),
            since: None,
            stdout: Some(true),
            stderr: Some(true),
            tail: None,
            timestamps: Some(false),
            until: None,
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct PauseContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for PauseContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("PauseContainerParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct RenameContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
    #[serde(rename = "name")]
    pub rename_to: String,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for RenameContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("RenameContainerParameter"),
            rename_to: String::from("RenameContainerParameter_renamed"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct RestartContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
    #[serde(rename = "t")]
    pub timeout: Option<u32>,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for RestartContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("RestartContainerParameter"),
            timeout: Some(0),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct RestoreContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
    #[serde(rename = "ignoreRootFS")]
    pub ignore_root_fs: Option<bool>,
    #[serde(rename = "ignoreStaticIP")]
    pub ignore_static_ip: Option<bool>,
    #[serde(rename = "igrnoreStaticMAC")]
    pub ignore_static_mac: Option<bool>,
    pub import: Option<bool>,
    pub keep: Option<bool>,
    #[serde(rename = "leaveRunning")]
    pub leave_running: Option<bool>,
    pub name: Option<String>,
    #[serde(rename = "printStats")]
    pub print_stats: Option<bool>,
    #[serde(rename = "tcpEstablished")]
    pub tcp_established: Option<bool>,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for RestoreContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("RestoreContainerParameter"),
            ignore_root_fs: Some(false),
            ignore_static_ip: Some(false),
            ignore_static_mac: Some(false),
            import: Some(false),
            keep: Some(false),
            leave_running: Some(false),
            name: None,
            print_stats: Some(false),
            tcp_established: Some(false),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ContainersStatsParameter {
    #[serde(rename = "containers")]
    pub container_names: Vec<String>,
    pub interval: Option<i32>,
    pub stream: Option<bool>,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ContainersStatsParameter {
    fn example() -> Self {
        Self {
            container_names: vec![
                String::from("ContainersStatsParameter1"),
                String::from("ContainersStatsParameter2"),
            ],
            interval: Some(5),
            stream: Some(true),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ListContainerProcessesParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
    pub delay: Option<u32>,
    pub ps_args: Option<String>,
    pub stream: Option<bool>,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ListContainerProcessesParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("ContainerStatsParameter"),
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
pub struct UnpauseContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for UnpauseContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("UnpauseContainerParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct MountContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for MountContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("MountContainerParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct UnmountContainerParameter {
    #[serde(skip_serializing)]
    pub container_name: String,
}
#[cfg(any(test, feature = "examples"))]
impl ExampleValues for UnmountContainerParameter {
    fn example() -> Self {
        Self {
            container_name: String::from("UnmountContainerParameter"),
        }
    }
}

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct PruneContainersParameter {
    pub filters: Option<HashMap<String, Vec<String>>>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for PruneContainersParameter {
    fn example() -> Self {
        let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
        filter_map.insert(
            String::from("label"),
            vec![String::from("PruneContainersParameter")],
        );
        Self {
            filters: Some(filter_map),
        }
    }
}

/// Internal representation of `PruneContainersParameter` since filters can't be serialized in a single step.
/// It needs to be serialized to json, then to query. `TryInto` tries to perform the serialisation into json.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct PruneContainersParameterQuery {
    pub filters: Option<String>,
}
impl TryFrom<PruneContainersParameter> for PruneContainersParameterQuery {
    type Error = PodtenderError;
    fn try_from(param: PruneContainersParameter) -> Result<Self, Self::Error> {
        let filters = utils::convert_from_map_to_json_string(param.filters)?;
        Ok(PruneContainersParameterQuery { filters })
    }
}

#[cfg(test)]
#[cfg(feature = "builder")]
mod container_parameter_types {
    use super::*;
    use serde_qs;

    #[test]
    #[cfg(feature = "builder")]
    fn delete_parameter() {
        let delete_parameter = DeleteContainerParameterBuilder::default()
            .force(true)
            .v(true)
            .build()
            .expect("Error building delete parameter.");

        assert_eq!(
            "force=true&v=true",
            serde_qs::to_string(&delete_parameter).expect("Error serializing DeleteParameter")
        );
    }

    #[test]
    #[cfg(feature = "builder")]
    fn start_parameter() {
        let start_parameter = StartContainerParameterBuilder::default()
            .detach_keys(String::from("ctrl-p,ctrl-q"))
            .build()
            .expect("Error building start parameter.");

        assert_eq!(
            "detachKeys=ctrl-p%2Cctrl-q",
            serde_qs::to_string(&start_parameter).expect("Error serializing StartParameter")
        );
    }

    #[test]
    #[cfg(feature = "builder")]
    fn start_default_parameter() {
        let start_parameter = StartContainerParameterBuilder::default()
            .build()
            .expect("Error building start parameter.");

        assert_eq!(
            "",
            serde_qs::to_string(&start_parameter).expect("Error serializing StartParameter")
        );
    }
    #[test]
    #[cfg(feature = "builder")]
    fn stop_parameter() {
        let stop_parameter = StopContainerParameterBuilder::default()
            .all(true)
            .ignore(false)
            .timeout(5)
            .build()
            .expect("Error building stop parameter.");

        assert_eq!(
            "all=true&Ignore=false&timeout=5",
            serde_qs::to_string(&stop_parameter).expect("Error serializing StopParameter")
        );
    }
}
