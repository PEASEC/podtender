use crate::networks::parameter_types::CreateNetworkParameter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RemoveNetworkResponse {
    #[serde(rename = "Err")]
    pub err: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NetworkConfig {
    #[serde(rename = "Bytes")]
    pub bytes: Option<String>,
    #[serde(rename = "Network")]
    pub network: Option<NetConf>,
    #[serde(rename = "bridge")]
    pub bridge: Option<String>,
    #[serde(rename = "hairpinMode")]
    pub hairpin_mode: Option<bool>,
    #[serde(rename = "ipMasq")]
    pub ip_masq: Option<bool>,
    #[serde(rename = "ipam")]
    pub ipam: Option<NetworkConfigIpam>,
    #[serde(rename = "isGateway")]
    pub is_gateway: Option<bool>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub capabilities: Option<HashMap<String, bool>>,
    pub backend: Option<String>,
    #[serde(rename = "domainName")]
    pub domain_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NetworkConfigIpam {
    pub ranges: Option<Vec<Vec<HashMap<String, String>>>>,
    pub subnet: Option<String>,
    pub routes: Option<Vec<HashMap<String, String>>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NetConf {
    #[serde(rename = "capabilities")]
    pub capabilities: Option<HashMap<String, bool>>,
    #[serde(rename = "cniVersion")]
    pub cni_version: Option<String>,
    #[serde(rename = "dns")]
    pub dns: Option<Dns>,
    #[serde(rename = "ipam")]
    pub ipam: Option<Ipam>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "prevResult")]
    pub prev_result: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Dns {
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    #[serde(rename = "nameservers")]
    pub nameservers: Option<Vec<String>>,
    #[serde(rename = "options")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "search")]
    pub search: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Ipam {
    #[serde(rename = "Config")]
    pub config: Option<Vec<IpamConfig>>,
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct IpamConfig {
    #[serde(rename = "AuxiliaryAddresses")]
    pub auxiliary_addresses: Option<HashMap<String, String>>,
    #[serde(rename = "Gateway")]
    pub gateway: Option<String>,
    #[serde(rename = "IPRange")]
    pub ip_range: Option<String>,
    #[serde(rename = "Subnet")]
    pub subnet: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PruneNetworksResponseEntry {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Error")]
    pub err: Option<String>,
}

pub type InspectNetworkResponse = CreateNetworkParameter;
pub type CreateNetworkResponse = CreateNetworkParameter;
pub type ListNetworksResponseEntry = CreateNetworkParameter;
