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
pub struct CreateVolumeParameter {
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    pub volume_name: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<HashMap<String, String>>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for CreateVolumeParameter {
    fn example() -> Self {
        let mut label: HashMap<String, String> = HashMap::new();
        label.insert(String::from("example"), String::from("yes"));
        Self {
            driver: Some(String::from("local")),
            labels: Some(label),
            volume_name: Some(String::from("CreateVolumeParameter")),
            options: None,
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct VolumeExistsParameter {
    pub volume_name: String,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for VolumeExistsParameter {
    fn example() -> Self {
        Self {
            volume_name: String::from("ExistsVolumeParameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct InspectVolumeParameter {
    pub volume_name: String,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for InspectVolumeParameter {
    fn example() -> Self {
        Self {
            volume_name: String::from("InspectVolumeParameter"),
        }
    }
}

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct RemoveVolumeParameter {
    #[serde(skip_serializing)]
    pub volume_name: String,
    pub force: Option<bool>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for RemoveVolumeParameter {
    fn example() -> Self {
        Self {
            volume_name: String::from("RemoveVolumeParameter"),
            force: Some(true),
        }
    }
}

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ListVolumesParameter {
    pub filters: Option<HashMap<String, Vec<String>>>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ListVolumesParameter {
    fn example() -> Self {
        let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
        filter_map.insert(String::from("label"), vec![String::from("example_list")]);
        Self {
            filters: Some(filter_map),
        }
    }
}

/// Internal representation of `ListVolumesParameter` since filters can't be serialized in a single step.
/// It needs to be serialized to json, then to query. `TryInto` tries to perform the serialisation into json.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct ListVolumesParameterQuery {
    pub filters: Option<String>,
}

impl TryFrom<ListVolumesParameter> for ListVolumesParameterQuery {
    type Error = PodtenderError;
    fn try_from(param: ListVolumesParameter) -> Result<Self, Self::Error> {
        let filters = utils::convert_from_map_to_json_string(param.filters)?;
        Ok(ListVolumesParameterQuery { filters })
    }
}

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct PruneVolumesParameter {
    pub filters: Option<HashMap<String, Vec<String>>>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for PruneVolumesParameter {
    fn example() -> Self {
        let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
        filter_map.insert(String::from("label"), vec![String::from("example_prune")]);
        Self {
            filters: Some(filter_map),
        }
    }
}

/// Internal representation of `ListVolumesParameter` since filters can't be serialized in a single step.
/// It needs to be serialized to json, then to query. `TryInto` tries to perform the serialisation into json.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct PruneVolumesParameterQuery {
    pub filters: Option<String>,
}

impl TryFrom<PruneVolumesParameter> for PruneVolumesParameterQuery {
    type Error = PodtenderError;
    fn try_from(param: PruneVolumesParameter) -> Result<Self, Self::Error> {
        let filters = utils::convert_from_map_to_json_string(param.filters)?;
        Ok(PruneVolumesParameterQuery { filters })
    }
}

#[cfg(test)]
#[cfg(feature = "builder")]
mod container_parameter_types {
    use super::*;
    use serde_json;

    #[test]
    #[cfg(feature = "builder")]
    fn create_volume_parameter() {
        let mut label = HashMap::new();
        label.insert(String::from("testkey"), String::from("testvalue"));

        let mut options = HashMap::new();
        options.insert(String::from("device"), String::from("tempfs"));
        let create_volume_parameter = CreateVolumeParameterBuilder::default()
            .driver(String::from("local"))
            .labels(label)
            .volume_name(String::from("somevolume"))
            .options(options)
            .build()
            .expect("Error building CreateVolumeParameter");

        assert_eq!(
            r#"{"Driver":"local","Labels":{"testkey":"testvalue"},"Name":"somevolume","Options":{"device":"tempfs"}}"#,
            serde_json::to_string(&create_volume_parameter)
                .expect("Error serializing DeleteParameter")
        );
    }
}
