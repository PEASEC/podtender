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
pub struct RemoveImageParameter {
    #[serde(skip_serializing)]
    pub image_name: String,
    pub force: Option<bool>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for RemoveImageParameter {
    fn example() -> Self {
        Self {
            image_name: String::from("remove_image_parameter"),
            force: Some(false),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ImageExistsParameter {
    #[serde(skip_serializing)]
    pub image_name: String,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ImageExistsParameter {
    fn example() -> Self {
        Self {
            image_name: String::from("exists_image_parameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ExportImageParameter {
    #[serde(skip_serializing)]
    pub image_name: String,
    pub format: Option<String>,
    pub compress: Option<bool>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ExportImageParameter {
    fn example() -> Self {
        Self {
            image_name: String::from("export_image_parameter"),
            format: Some(String::from("oci-archive")),
            compress: None,
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct InspectImageParameter {
    #[serde(skip_serializing)]
    pub image_name: String,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for InspectImageParameter {
    fn example() -> Self {
        Self {
            image_name: String::from("inspect_image_parameter"),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ImportImageParameter {
    pub changes: Option<Vec<String>>,
    pub message: Option<String>,
    pub reference: Option<String>,
    pub url: Option<String>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ImportImageParameter {
    fn example() -> Self {
        Self {
            changes: None,
            message: Some(String::from("import_image_parameter_message")),
            reference: Some(String::from("import_image_parameter_reference:reference")),
            url: None,
        }
    }
}

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct ListImagesParameter {
    pub all: Option<bool>,
    pub filters: Option<HashMap<String, Vec<String>>>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for ListImagesParameter {
    fn example() -> Self {
        let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
        filter_map.insert(
            String::from("reference"),
            vec![String::from("list_image_parameter")],
        );
        Self {
            all: Some(false),
            filters: Some(filter_map),
        }
    }
}

/// Internal representation of `ListImagesParameter` since filters can't be serialized in a single step.
/// It needs to be serialized to json, then to query. `TryInto` tries to perform the serialisation into json.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct ListImagesParameterQuery {
    pub all: Option<bool>,
    pub filters: Option<String>,
}

impl TryFrom<ListImagesParameter> for ListImagesParameterQuery {
    type Error = PodtenderError;
    fn try_from(param: ListImagesParameter) -> Result<Self, Self::Error> {
        let filters = utils::convert_from_map_to_json_string(param.filters)?;
        Ok(ListImagesParameterQuery {
            all: param.all,
            filters,
        })
    }
}

//query
#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct PruneImagesParameter {
    pub all: Option<bool>,
    pub external: Option<bool>,
    pub filters: Option<HashMap<String, Vec<String>>>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for PruneImagesParameter {
    fn example() -> Self {
        let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
        filter_map.insert(
            String::from("label"),
            vec![String::from("PruneImagesParameter")],
        );
        Self {
            all: Some(true),
            external: Some(false),
            filters: Some(filter_map),
        }
    }
}

/// Internal representation of `PruneImagesParameter` since filters can't be serialized in a single step.
/// It needs to be serialized to json, then to query. `TryInto` tries to perform the serialisation into json.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct PruneImagesParameterQuery {
    pub all: Option<bool>,
    pub filters: Option<String>,
}

impl TryFrom<PruneImagesParameter> for PruneImagesParameterQuery {
    type Error = PodtenderError;
    fn try_from(param: PruneImagesParameter) -> Result<Self, Self::Error> {
        let filters = utils::convert_from_map_to_json_string(param.filters)?;
        Ok(PruneImagesParameterQuery {
            all: param.all,
            filters,
        })
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct PullImagesParameter {
    #[serde(skip_serializing)]
    pub x_registry_auth_header: Option<String>,
    #[serde(rename = "allTags")]
    pub all_tags: Option<bool>,
    #[serde(rename = "Arch")]
    pub arch: Option<String>,
    pub credentials: Option<String>,
    #[serde(rename = "OS")]
    pub os: Option<String>,
    pub policy: Option<String>,
    pub quiet: Option<bool>,
    pub reference: Option<String>,
    #[serde(rename = "tlsVerify")]
    pub tls_verify: Option<bool>,
    #[serde(rename = "Variant")]
    pub variant: Option<String>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for PullImagesParameter {
    fn example() -> Self {
        Self {
            x_registry_auth_header: None,
            reference: Some(String::from("docker://docker.io/library/hello-world")),
            quiet: Some(false),
            credentials: None,
            arch: Some(String::from("amd64")),
            os: Some(String::from("linux")),
            variant: None,
            policy: Some(String::from("always")),
            tls_verify: Some(true),
            all_tags: Some(false),
        }
    }
}

#[skip_serializing_none]
#[cfg_attr(feature = "builder", derive(Builder))]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
pub struct SearchImagesParameter {
    pub filters: Option<HashMap<String, Vec<String>>>,
    pub limit: Option<i32>,
    #[serde(rename = "listTags")]
    pub list_tags: Option<bool>,
    pub term: Option<String>,
    #[serde(rename = "tlsVerify")]
    pub tls_verify: Option<bool>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for SearchImagesParameter {
    fn example() -> Self {
        let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
        filter_map.insert(String::from("is-official"), vec![String::from("true")]);
        filter_map.insert(String::from("is-automated"), vec![String::from("false")]);
        // podman has started filtering for exactly the specified stars with v4.0.0
        //filter_map.insert(String::from("stars"), vec![String::from("1")]);
        Self {
            term: Some(String::from("hello-world")),
            limit: None,
            list_tags: Some(false),
            filters: Some(filter_map),
            tls_verify: Some(false),
        }
    }
}

/// Internal representation of `SearchImagesParameter` since filters can't be serialized in a single step.
/// It needs to be serialized to json, then to query. `TryInto` tries to perform the serialisation into json.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct SearchImagesParameterQuery {
    pub limit: Option<i32>,
    #[serde(rename = "listTags")]
    pub list_tags: Option<bool>,
    pub term: Option<String>,
    #[serde(rename = "tlsVerify")]
    pub tls_verify: Option<bool>,
    pub filters: Option<String>,
}

impl TryFrom<SearchImagesParameter> for SearchImagesParameterQuery {
    type Error = PodtenderError;
    fn try_from(param: SearchImagesParameter) -> Result<Self, Self::Error> {
        let filters = utils::convert_from_map_to_json_string(param.filters)?;
        Ok(SearchImagesParameterQuery {
            limit: param.limit,
            list_tags: param.list_tags,
            term: param.term,
            tls_verify: param.tls_verify,
            filters,
        })
    }
}
