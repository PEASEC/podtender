use crate::containers::parameter_types::{PortMapping, Schema2HealthConfig};
use crate::pods::response_types::{
    ErrIdResponse, InspectDevice, InspectHostPort, InspectMount, ListPodProcessesResponse,
};
use crate::volumes::response_types::ErrIdSizeResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CreateContainerResponse {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Warnings")]
    pub warnings: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ListContainersResponseEntry {
    #[serde(rename = "AutoRemove")]
    pub auto_remove: Option<bool>,
    #[serde(rename = "Command")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "Created")]
    pub created: Option<String>,
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<String>,
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i32>,
    #[serde(rename = "Exited")]
    pub exited: Option<bool>,
    #[serde(rename = "ExitedAt")]
    pub exited_at: Option<i64>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Image")]
    pub image: Option<String>,
    #[serde(rename = "ImageID")]
    pub image_id: Option<String>,
    #[serde(rename = "IsInfra")]
    pub is_infra: Option<bool>,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Mounts")]
    pub mounts: Option<Vec<String>>,
    #[serde(rename = "Names")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "Namespaces")]
    pub namespaces: Option<ListContainerNamespaces>,
    #[serde(rename = "Networks")]
    pub networks: Option<Vec<String>>,
    #[serde(rename = "Pid")]
    pub pid: Option<i64>,
    #[serde(rename = "Pod")]
    pub pod: Option<String>,
    #[serde(rename = "PodName")]
    pub pod_name: Option<String>,
    #[serde(rename = "Ports")]
    pub ports: Option<Vec<PortMapping>>,
    #[serde(rename = "Size")]
    pub size: Option<ContainerSize>,
    #[serde(rename = "StartedAt")]
    pub started_at: Option<i64>,
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ListContainerNamespaces {
    #[serde(rename = "Cgroup")]
    pub c_group: Option<String>,
    #[serde(rename = "Ipc")]
    pub ipc: Option<String>,
    #[serde(rename = "Mnt")]
    pub mnt: Option<String>,
    #[serde(rename = "Net")]
    pub net: Option<String>,
    #[serde(rename = "Pidns")]
    pub pidns: Option<String>,
    #[serde(rename = "User")]
    pub user: Option<String>,
    #[serde(rename = "Uts")]
    pub uts: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ContainerSize {
    #[serde(rename = "rootFsSize")]
    pub root_fs_size: Option<i64>,
    #[serde(rename = "rwSize")]
    pub rw_size: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct HealthcheckContainerResponse {
    #[serde(rename = "FailingStreak")]
    pub failing_streak: Option<i64>,
    #[serde(rename = "Log")]
    pub log: Option<Vec<HealthCheckLog>>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}
type HealthCheckResults = HealthcheckContainerResponse;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct HealthCheckLog {
    #[serde(rename = "End")]
    pub end: Option<String>,
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i64>,
    #[serde(rename = "Output")]
    pub output: Option<String>,
    #[serde(rename = "Start")]
    pub start: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectContainerResponse {
    #[serde(rename = "AppArmorProfile")]
    pub app_armor_profile: Option<String>,
    #[serde(rename = "Args")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "BoundingCaps")]
    pub bounding_caps: Option<Vec<String>>,
    #[serde(rename = "Config")]
    pub config: Option<InspectContainerConfig>,
    #[serde(rename = "ConmonPidFile")]
    pub conmon_pid_file: Option<String>,
    #[serde(rename = "Created")]
    pub created: Option<String>,
    #[serde(rename = "Dependencies")]
    pub dependencies: Option<Vec<String>>,
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    #[serde(rename = "EffectiveCaps")]
    pub effective_caps: Option<Vec<String>>,
    #[serde(rename = "ExecIDs")]
    pub exec_ids: Option<Vec<String>>,
    #[serde(rename = "ExitCommand")]
    pub exit_command: Option<Vec<String>>,
    #[serde(rename = "GraphDriver")]
    pub graph_driver: Option<DriverData>,
    #[serde(rename = "HostConfig")]
    pub host_config: Option<InspectContainerHostConfig>,
    #[serde(rename = "HostnamePath")]
    pub hostname_path: Option<String>,
    #[serde(rename = "HostsPath")]
    pub hosts_path: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Image")]
    pub image: Option<String>,
    #[serde(rename = "ImageDigest")]
    pub image_digest: Option<String>,
    #[serde(rename = "ImageName")]
    pub image_name: Option<String>,
    #[serde(rename = "IsInfra")]
    pub is_infra: Option<bool>,
    #[serde(rename = "IsService")]
    pub is_service: Option<bool>,
    #[serde(rename = "MountLabel")]
    pub mount_label: Option<String>,
    #[serde(rename = "Mounts")]
    pub mounts: Option<Vec<InspectMount>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: Option<InspectNetworkSettings>,
    #[serde(rename = "OCIConfigPath")]
    pub oci_config_path: Option<String>,
    #[serde(rename = "OCIRuntime")]
    pub oci_runtime: Option<String>,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "PidFile")]
    pub pid_file: Option<String>,
    #[serde(rename = "Pod")]
    pub pod: Option<String>,
    #[serde(rename = "ProcessLabel")]
    pub process_label: Option<String>,
    #[serde(rename = "ResolvConfPath")]
    pub resolv_conf_path: Option<String>,
    #[serde(rename = "RestartCount")]
    pub restart_count: Option<i32>,
    #[serde(rename = "Rootfs")]
    pub rootfs: Option<String>,
    #[serde(rename = "SizeRootFs")]
    pub size_rootfs: Option<i64>,
    #[serde(rename = "SizeRw")]
    pub size_rw: Option<i64>,
    #[serde(rename = "State")]
    pub state: Option<InspectContainerState>,
    #[serde(rename = "StaticDir")]
    pub static_dir: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectContainerConfig {
    #[serde(rename = "Annotations")]
    pub annotations: Option<HashMap<String, String>>,
    #[serde(rename = "AttachStderr")]
    pub attach_stderr: Option<bool>,
    #[serde(rename = "AttachStdin")]
    pub attach_stdin: Option<bool>,
    #[serde(rename = "AttachStdout")]
    pub attach_stdout: Option<bool>,
    #[serde(rename = "Cmd")]
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "CreateCommand")]
    pub create_command: Option<Vec<String>>,
    #[serde(rename = "Domainname")]
    pub domainname: Option<String>,
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Option<String>,
    #[serde(rename = "Env")]
    pub env: Option<Vec<String>>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Option<Schema2HealthConfig>,
    #[serde(rename = "HealthcheckOnFailureAction")]
    pub healthcheck_on_failure_action: Option<String>,
    #[serde(rename = "Hostname")]
    pub hostname: Option<String>,
    #[serde(rename = "Image")]
    pub image: Option<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "OnBuild")]
    pub on_build: Option<String>,
    #[serde(rename = "OpenStdin")]
    pub open_stdin: Option<bool>,
    #[serde(rename = "Passwd")]
    pub passwd: Option<bool>,
    #[serde(rename = "sdNotifyMode")]
    pub sd_notify_mode: Option<String>,
    #[serde(rename = "sdNotifySocket")]
    pub sd_notify_socket: Option<String>,
    #[serde(rename = "Secrets")]
    pub secrets: Option<Vec<InspectSecret>>,
    #[serde(rename = "StdinOnce")]
    pub stdin_once: Option<bool>,
    #[serde(rename = "StopSignal")]
    pub stop_signal: Option<u64>,
    #[serde(rename = "StopTimeout")]
    pub stop_timeout: Option<u64>,
    #[serde(rename = "SystemdMode")]
    pub systemd_mode: Option<bool>,
    #[serde(rename = "Timeout")]
    pub timeout: Option<u64>,
    #[serde(rename = "Timezone")]
    pub timezone: Option<String>,
    #[serde(rename = "Tty")]
    pub tty: Option<bool>,
    #[serde(rename = "Umask")]
    pub umask: Option<String>,
    #[serde(rename = "User")]
    pub user: Option<String>,
    #[serde(rename = "Volumes")]
    pub volumes: Option<HashMap<String, String>>,
    #[serde(rename = "WorkingDir")]
    pub working_dir: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectSecret {
    #[serde(rename = "GID")]
    pub gid: Option<u32>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "Mode")]
    pub mode: Option<u32>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "UID")]
    pub uid: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DriverData {
    #[serde(rename = "Data")]
    pub data: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectContainerHostConfig {
    #[serde(rename = "AutoRemove")]
    pub auto_remove: Option<bool>,
    #[serde(rename = "Binds")]
    pub binds: Option<Vec<String>>,
    #[serde(rename = "BlkioDeviceReadBps")]
    pub blkio_device_read_bps: Option<Vec<InspectBlkioThrottleDevice>>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    pub blkio_device_read_iops: Option<InspectBlkioThrottleDevice>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    pub blkio_device_write_bps: Option<InspectBlkioThrottleDevice>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    pub blkio_device_write_iops: Option<InspectBlkioThrottleDevice>,
    #[serde(rename = "BlkioWeight")]
    pub blkio_weight: Option<u16>,
    #[serde(rename = "BlkioWeightDevice")]
    pub blkio_weight_device: Option<InspectBlkioWeightDevice>,
    #[serde(rename = "CapAdd")]
    pub cap_add: Option<Vec<String>>,
    #[serde(rename = "CapDrop")]
    pub cap_drop: Option<Vec<String>>,
    #[serde(rename = "Cgroup")]
    pub cgroup: Option<String>,
    #[serde(rename = "CgroupConf")]
    pub cgroup_conf: Option<HashMap<String, String>>,
    #[serde(rename = "CgroupManager")]
    pub cgroup_manager: Option<String>,
    #[serde(rename = "CgroupMode")]
    pub cgroup_mode: Option<String>,
    #[serde(rename = "CgroupParent")]
    pub cgroup_parent: Option<String>,
    #[serde(rename = "Cgroups")]
    pub cgroups: Option<String>,
    #[serde(rename = "ConsoleSize")]
    pub console_size: Option<Vec<u64>>,
    #[serde(rename = "ContainerIDFile")]
    pub container_id_file: Option<String>,
    #[serde(rename = "CpuCount")]
    pub cpu_count: Option<u64>,
    #[serde(rename = "CpuPercent")]
    pub cpu_percent: Option<u64>,
    #[serde(rename = "CpuPeriod")]
    pub cpu_period: Option<u64>,
    #[serde(rename = "CpuQuota")]
    pub cpu_quota: Option<u64>,
    #[serde(rename = "CpuRealtimePeriod")]
    pub cpu_realtime_period: Option<u64>,
    #[serde(rename = "CpuRealtimeRuntime")]
    pub cpu_realtime_runtime: Option<u64>,
    #[serde(rename = "CpusetCpus")]
    pub cpuset_cpus: Option<String>,
    #[serde(rename = "CpusetMems")]
    pub cpuset_mems: Option<String>,
    #[serde(rename = "CpuShares")]
    pub cpu_shares: Option<u64>,
    #[serde(rename = "Devices")]
    pub devices: Option<Vec<InspectDevice>>,
    #[serde(rename = "DiskQuota")]
    pub disk_quota: Option<u64>,
    #[serde(rename = "Dns")]
    pub dns: Option<Vec<String>>,
    #[serde(rename = "DnsOptions")]
    pub dns_options: Option<Vec<String>>,
    #[serde(rename = "DnsSearch")]
    pub dns_search: Option<Vec<String>>,
    #[serde(rename = "ExtraHosts")]
    pub extra_hosts: Option<Vec<String>>,
    #[serde(rename = "GroupAdd")]
    pub group_add: Option<Vec<String>>,
    #[serde(rename = "Init")]
    pub init: Option<bool>,
    #[serde(rename = "IOMaximumBandwidth")]
    pub io_maximum_bandwidth: Option<u64>,
    #[serde(rename = "IOMaximumIOps")]
    pub io_maximum_iops: Option<u64>,
    #[serde(rename = "IpcMode")]
    pub ipc_mode: Option<String>,
    #[serde(rename = "Isolation")]
    pub isolation: Option<String>,
    #[serde(rename = "KernelMemory")]
    pub kernel_memory: Option<i64>,
    #[serde(rename = "Links")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "LogConfig")]
    pub log_config: Option<InspectLogConfig>,
    #[serde(rename = "Memory")]
    pub memory: Option<i64>,
    #[serde(rename = "MemoryReservation")]
    pub memory_reservation: Option<i64>,
    #[serde(rename = "MemorySwap")]
    pub memory_swap: Option<i64>,
    #[serde(rename = "MemorySwappiness")]
    pub memory_swappiness: Option<i64>,
    #[serde(rename = "NanoCpus")]
    pub nano_cpus: Option<i64>,
    #[serde(rename = "NetworkMode")]
    pub network_mode: Option<String>,
    #[serde(rename = "OomKillDisable")]
    pub oom_kill_disable: Option<bool>,
    #[serde(rename = "OomScoreAdj")]
    pub oom_score_adj: Option<i64>,
    #[serde(rename = "PidMode")]
    pub pid_mode: Option<String>,
    #[serde(rename = "PidsLimit")]
    pub pids_limit: Option<i64>,
    #[serde(rename = "PortBindings")]
    pub port_bindings: Option<HashMap<String, Vec<InspectHostPort>>>,
    #[serde(rename = "Privileged")]
    pub privileged: Option<bool>,
    #[serde(rename = "PublishAllPorts")]
    pub publish_all_ports: Option<bool>,
    #[serde(rename = "ReadonlyRootfs")]
    pub readonly_rootfs: Option<bool>,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: Option<InspectRestartPolicy>,
    #[serde(rename = "Runtime")]
    pub runtime: Option<String>,
    #[serde(rename = "SecurityOpt")]
    pub security_opt: Option<Vec<String>>,
    #[serde(rename = "ShmSize")]
    pub shm_size: Option<i64>,
    #[serde(rename = "Tmpfs")]
    pub tmpfs: Option<HashMap<String, String>>,
    #[serde(rename = "Ulimits")]
    pub ulimits: Option<Vec<InspectUlimit>>,
    #[serde(rename = "UsernsMode")]
    pub userns_mode: Option<String>,
    #[serde(rename = "UTSMode")]
    pub uts_mode: Option<String>,
    #[serde(rename = "VolumeDriver")]
    pub volume_driver: Option<String>,
    #[serde(rename = "VolumesFrom")]
    pub volumes_from: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectBlkioThrottleDevice {
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "Rate")]
    pub rate: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectBlkioWeightDevice {
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "Rate")]
    pub rate: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectLogConfig {
    #[serde(rename = "Config")]
    pub config: Option<HashMap<String, String>>,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<String>,
    #[serde(rename = "Tag")]
    pub tag: Option<String>,
    #[serde(rename = "Type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectRestartPolicy {
    #[serde(rename = "MaximumRetryCount")]
    pub maximum_retry_count: Option<u64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectUlimit {
    #[serde(rename = "Hard")]
    pub hard: Option<i64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Soft")]
    pub soft: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectNetworkSettings {
    #[serde(rename = "AdditionalMacAddresses")]
    pub additional_mac_addresses: Option<Vec<String>>,
    #[serde(rename = "Bridge")]
    pub bridge: Option<String>,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "Gateway")]
    pub gateway: Option<String>,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: Option<String>,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: Option<i64>,
    #[serde(rename = "HairpinMode")]
    pub hairpin_mode: Option<bool>,
    #[serde(rename = "IPAddress")]
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: Option<i64>,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: Option<String>,
    #[serde(rename = "LinkLocalIPv6Address")]
    pub link_local_ipv6_address: Option<String>,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    pub link_local_ipv6_prefix_len: Option<i64>,
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,
    #[serde(rename = "Networks")]
    pub networks: Option<HashMap<String, InspectAdditionalNetwork>>,
    #[serde(rename = "Ports")]
    pub ports: Option<HashMap<String, InspectHostPort>>,
    #[serde(rename = "SandboxID")]
    pub sandbox_id: Option<String>,
    #[serde(rename = "SandboxKey")]
    pub sandbox_key: Option<String>,
    #[serde(rename = "SecondaryIPAddresses")]
    pub secondary_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "SecondaryIpv6Addresses")]
    pub secondary_ipv6_addresses: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectAdditionalNetwork {
    #[serde(rename = "AdditionalMacAddresses")]
    pub additional_mac_addresses: Option<Vec<String>>,
    #[serde(rename = "Aliases")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "Gateway")]
    pub gateway: Option<String>,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: Option<String>,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: Option<i64>,
    #[serde(rename = "IPAddress")]
    pub ip_address: Option<String>,
    #[serde(rename = "IPAMConfig")]
    pub ipam_config: Option<HashMap<String, String>>,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: Option<i64>,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: Option<String>,
    #[serde(rename = "Links")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,
    #[serde(rename = "NetworkID")]
    pub network_id: Option<String>,
    #[serde(rename = "SecondaryIPAddresses")]
    pub secondary_ip_addresses: Option<Vec<String>>,
    #[serde(rename = "SecondaryIpv6Addresses")]
    pub secondary_ipv6_addresses: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InspectContainerState {
    #[serde(rename = "CgroupPath")]
    pub cgroup_path: Option<String>,
    #[serde(rename = "Checkpointed")]
    pub checkpointed: Option<bool>,
    #[serde(rename = "CheckpointedAt")]
    pub checkpointed_at: Option<String>,
    #[serde(rename = "CheckpointLog")]
    pub checkpoint_log: Option<String>,
    #[serde(rename = "CheckpointedPath")]
    pub checkpoint_path: Option<String>,
    #[serde(rename = "ConmonPid")]
    pub conmon_pid: Option<i64>,
    #[serde(rename = "Dead")]
    pub dead: Option<bool>,
    #[serde(rename = "Error")]
    pub error: Option<String>,
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i32>,
    #[serde(rename = "FinishedAt")]
    pub finished_at: Option<String>,
    #[serde(rename = "Health")]
    pub health: Option<HealthCheckResults>,
    #[serde(rename = "OciVersion")]
    pub oci_version: Option<String>,
    #[serde(rename = "OOMKilled")]
    pub oom_killed: Option<bool>,
    #[serde(rename = "Paused")]
    pub paused: Option<bool>,
    #[serde(rename = "Pid")]
    pub pid: Option<i64>,
    #[serde(rename = "Restarting")]
    pub restarting: Option<bool>,
    #[serde(rename = "Restored")]
    pub restored: Option<bool>,
    #[serde(rename = "RestoredAt")]
    pub restored_at: Option<String>,
    #[serde(rename = "RestoreLog")]
    pub restore_log: Option<String>,
    #[serde(rename = "Running")]
    pub running: Option<bool>,
    #[serde(rename = "StartedAt")]
    pub started_at: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)] //Eq,
#[serde(deny_unknown_fields)]
pub struct ContainerStatsResponse {
    #[serde(rename = "Error")]
    pub error: Option<String>,
    #[serde(rename = "Stats")]
    pub stats: Option<Vec<ContainerStatsResponseInner>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)] //Eq,
#[serde(deny_unknown_fields)]
pub struct ContainerStatsResponseInner {
    #[serde(rename = "AvgCPU")]
    pub avg_cpu: Option<f64>,
    #[serde(rename = "BlockInput")]
    pub block_input: Option<u64>,
    #[serde(rename = "BlockOutput")]
    pub block_output: Option<u64>,
    #[serde(rename = "ContainerID")]
    pub container_id: Option<String>,
    #[serde(rename = "CPU")]
    pub cpu: Option<f64>,
    #[serde(rename = "CPUNano")]
    pub cpu_nano: Option<u64>,
    #[serde(rename = "CPUSystemNano")]
    pub cpu_system_nano: Option<u64>,
    #[serde(rename = "DataPoints")]
    pub data_points: Option<i64>,
    #[serde(rename = "Duration")]
    pub duration: Option<u64>,
    #[serde(rename = "MemLimit")]
    pub mem_limit: Option<u64>,
    #[serde(rename = "MemPerc")]
    pub mem_perc: Option<f64>,
    #[serde(rename = "MemUsage")]
    pub mem_usage: Option<u64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "NetInput")]
    pub net_input: Option<u64>,
    #[serde(rename = "NetOutput")]
    pub net_output: Option<u64>,
    #[serde(rename = "PerCPU")]
    pub per_cpu: Option<Vec<u64>>,
    #[serde(rename = "PIDs")]
    pub pids: Option<u64>,
    #[serde(rename = "SystemNano")]
    pub system_nano: Option<u64>,
    #[serde(rename = "UpTime")]
    pub up_time: Option<u64>,
}

pub type PruneContainerResponseEntry = ErrIdSizeResponse;

pub type ContainerDeleteResponseEntry = ErrIdResponse;

pub type ListContainerProcessesResponse = ListPodProcessesResponse;
