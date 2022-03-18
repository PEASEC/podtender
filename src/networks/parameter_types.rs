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

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct RemoveNetworkParameter {
    #[serde(skip_serializing)]
    pub network_name: String,
    pub force: Option<bool>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for RemoveNetworkParameter {
    fn example() -> Self {
        Self {
            network_name: String::from("RemoveNetworkParameter"),
            force: Some(true),
        }
    }
}

//json
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ConnectContainerParameter {
    #[serde(skip_serializing)]
    pub network_name: String,
    pub aliases: Option<Vec<String>>,
    pub container: String,
    pub interface_name: Option<String>,
    pub static_ips: Option<Vec<String>>,
    pub static_mac: Option<String>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ConnectContainerParameter {
    fn example() -> Self {
        Self {
            network_name: "ConnectContainerParameterNetwork".to_string(),
            aliases: Some(vec![
                "connect_here".to_owned(),
                "connect_here_too".to_owned(),
            ]),
            container: "ConnectContainerParameter".to_owned(),
            interface_name: None,
            static_ips: Some(vec!["192.168.123.10".to_owned()]),
            static_mac: Some("4e:42:36:e8:84:35".to_owned()),
        }
    }
}

//json
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct DisconnectContainerParameter {
    #[serde(skip_serializing)]
    pub network_name: String,
    #[serde(rename = "Container")]
    pub container: String,
    #[serde(rename = "Force")]
    pub force: Option<bool>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for DisconnectContainerParameter {
    fn example() -> Self {
        Self {
            network_name: String::from("DisconnectContainerParameter"),
            container: String::from("DisconnectContainerParameterContainer"),
            force: Some(true),
        }
    }
}

// also used as InspectNetworResponse, ListNetworksResponseElement and CreateNetworkResponse
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
#[serde(deny_unknown_fields)]
pub struct CreateNetworkParameter {
    pub created: Option<String>,
    pub dns_enabled: Option<bool>,
    pub driver: Option<String>,
    pub id: Option<String>,
    pub internal: Option<bool>,
    pub ipam_options: Option<HashMap<String, String>>,
    pub ipv6_enabled: Option<bool>,
    pub labels: Option<HashMap<String, String>>,
    pub name: Option<String>,
    pub network_interface: Option<String>,
    pub options: Option<HashMap<String, String>>,
    pub subnets: Option<Vec<Subnet>>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for CreateNetworkParameter {
    fn example() -> Self {
        Self {
            created: None,
            dns_enabled: Some(true),
            driver: Some("bridge".to_owned()),
            id: None,
            internal: None,
            ipam_options: None,
            ipv6_enabled: None,
            labels: None,
            name: Some("CreateNetworkParameter".to_owned()),
            network_interface: None,
            options: None,
            subnets: None,
        }
    }
}

// also used in InspectNetworkResponse
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
#[serde(deny_unknown_fields)]
pub struct Subnet {
    pub gateway: Option<String>,
    pub lease_range: Option<LeaseRange>,
    pub subnet: Option<String>,
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct LeaseRange {
    pub end_ip: Option<String>,
    pub start_ip: Option<String>,
}

//json
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct IpNet {
    #[serde(rename = "IP")]
    pub ip: Option<Vec<u8>>,
    #[serde(rename = "Mask")]
    pub mask: Option<Vec<u8>>,
}

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ListNetworksParameter {
    pub filters: Option<HashMap<String, Vec<String>>>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ListNetworksParameter {
    fn example() -> Self {
        let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
        filter_map.insert(String::from("label"), vec![String::from("example_list")]);
        Self {
            filters: Some(filter_map),
        }
    }
}

/// Internal representation of `ListNetworksParameter` since filters can't be serialized in a single step.
/// It needs to be serialized to json, then to query. `TryInto` tries to perform the serialisation into json.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct ListNetworksParameterQuery {
    pub filters: Option<String>,
}
impl TryFrom<ListNetworksParameter> for ListNetworksParameterQuery {
    type Error = PodtenderError;
    fn try_from(param: ListNetworksParameter) -> Result<Self, Self::Error> {
        let filters = utils::convert_from_map_to_json_string(param.filters)?;
        Ok(ListNetworksParameterQuery { filters })
    }
}

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct PruneNetworksParameter {
    pub filters: Option<HashMap<String, Vec<String>>>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for PruneNetworksParameter {
    fn example() -> Self {
        let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
        filter_map.insert(String::from("label"), vec![String::from("example_prune")]);
        Self {
            filters: Some(filter_map),
        }
    }
}

/// Internal representation of `ListNetworksParameter` since filters can't be serialized in a single step.
/// It needs to be serialized to json, then to query. `TryInto` tries to perform the serialisation into json.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct PruneNetworksParameterQuery {
    pub filters: Option<String>,
}
impl TryFrom<PruneNetworksParameter> for PruneNetworksParameterQuery {
    type Error = PodtenderError;
    fn try_from(param: PruneNetworksParameter) -> Result<Self, Self::Error> {
        let filters = utils::convert_from_map_to_json_string(param.filters)?;
        Ok(PruneNetworksParameterQuery { filters })
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct NetworkExistsParameter {
    pub network_name: String,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for NetworkExistsParameter {
    fn example() -> Self {
        Self {
            network_name: String::from("ExistsNetworkParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct InspectNetworkParameter {
    pub network_name: String,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for InspectNetworkParameter {
    fn example() -> Self {
        Self {
            network_name: String::from("InspectNetworkParameter"),
        }
    }
}
