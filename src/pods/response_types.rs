use crate::containers::response_types::InspectBlkioThrottleDevice;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreatePodResponse {
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectPodResponse {
    #[serde(rename = "CgroupParent")]
    pub c_group_parent: Option<String>,
    #[serde(rename = "CgroupPath")]
    pub c_group_path: Option<String>,
    #[serde(rename = "Containers")]
    pub containers: Option<Vec<InspectPodContainerInfo>>,
    pub cpu_period: Option<u64>,
    pub cpu_quota: Option<i64>,
    pub cpusets_cpu: Option<String>,
    #[serde(rename = "CreateCgroup")]
    pub create_cgoup: Option<bool>,
    #[serde(rename = "CreateCommand")]
    pub create_command: Option<Vec<String>>,
    #[serde(rename = "Created")]
    pub created: Option<String>,
    #[serde(rename = "CreateInfra")]
    pub create_infra: Option<bool>,
    pub device_read_bps: Option<InspectBlkioThrottleDevice>,
    pub devices: Option<Vec<InspectDevice>>,
    #[serde(rename = "Hostname")]
    pub hostname: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "InfraConfig")]
    pub infra_config: Option<InspectPodInfraConfig>,
    #[serde(rename = "InfraContainerID")]
    pub infra_container_id: Option<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    pub mounts: Option<Vec<InspectMount>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "NumContainers")]
    pub num_containers: Option<u64>,
    pub security_opt: Option<Vec<String>>,
    #[serde(rename = "SharedNamespaces")]
    pub shared_namespaces: Vec<String>,
    #[serde(rename = "State")]
    pub state: Option<String>,
    pub volumes_from: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectPodContainerInfo {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    pub state: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectDevice {
    #[serde(rename = "CgroupPermissions")]
    pub cgroup_permissions: Option<String>,
    #[serde(rename = "PathInContainer")]
    pub path_in_container: Option<String>,
    #[serde(rename = "PathOnHost")]
    pub path_on_host: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectPodInfraConfig {
    pub cpu_period: Option<u64>,
    pub cpu_quota: Option<i64>,
    pub spuset_cpus: Option<String>,
    #[serde(rename = "DNSOption")]
    pub dns_option: Option<Vec<String>>,
    #[serde(rename = "DNSSearch")]
    pub dns_search: Option<Vec<String>>,
    #[serde(rename = "DNSServer")]
    pub dns_server: Option<Vec<String>>,
    #[serde(rename = "HostAdd")]
    pub host_add: Option<Vec<String>>,
    #[serde(rename = "HostNetwork")]
    pub host_network: Option<bool>,
    #[serde(rename = "NetworkOptions")]
    pub network_options: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "Networks")]
    pub networks: Option<Vec<String>>,
    #[serde(rename = "NoManageHosts")]
    pub no_manage_hosts: Option<bool>,
    #[serde(rename = "NoManageResolvConf")]
    pub no_manage_resolv_conf: Option<bool>,
    pub pid_ns: Option<String>,
    #[serde(rename = "PortBindings")]
    pub port_bindings: Option<HashMap<String, Vec<InspectHostPort>>>,
    #[serde(rename = "StaticIP")]
    pub static_ip: Option<String>,
    #[serde(rename = "StaticMAC")]
    pub static_mac: Option<String>,
    pub userns: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectHostPort {
    #[serde(rename = "HostIp")]
    pub host_ip: Option<String>,
    #[serde(rename = "HostPort")]
    pub host_port: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectMount {
    #[serde(rename = "Destination")]
    pub destination: Option<String>,
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    #[serde(rename = "Mode")]
    pub mode: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "Propagation")]
    pub propagation: Option<String>,
    #[serde(rename = "RW")]
    pub rw: Option<bool>,
    #[serde(rename = "Source")]
    pub source: Option<String>,
    #[serde(rename = "Type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ListPodProcessesResponse {
    #[serde(rename = "Processes")]
    pub processes: Option<Vec<Vec<String>>>,
    #[serde(rename = "Titles")]
    pub titles: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ListPodsResponseEntry {
    #[serde(rename = "Cgroup")]
    pub cgroup: Option<String>,
    #[serde(rename = "Containers")]
    pub containers: Option<Vec<ListPodContainer>>,
    #[serde(rename = "Created")]
    pub created: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "InfraId")]
    pub infra_id: Option<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "Networks")]
    pub networks: Option<Vec<String>>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ListPodContainer {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Names")]
    pub names: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ErrIdResponse {
    #[serde(rename = "Err")]
    pub err: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

pub type RemovePodResponse = ErrIdResponse;
pub type PrunePodsResponse = ErrIdResponse;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ErrsIdResponse {
    #[serde(rename = "Errs")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

pub type UnpausePodResponse = ErrsIdResponse;
pub type StopPodResponse = ErrsIdResponse;
pub type StartPodResponse = ErrsIdResponse;
pub type RestartPodResponse = ErrsIdResponse;
pub type PausePodResponse = ErrsIdResponse;
pub type KillPodResponse = ErrsIdResponse;
