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
pub struct EventsParameter {
    pub filters: Option<HashMap<String, Vec<String>>>,
    pub since: Option<String>,
    #[cfg_attr(feature = "builder", builder(default = "true"))]
    pub stream: bool,
    pub until: Option<String>,
}

#[cfg(any(test, feature = "examples"))]
impl ExampleValues for EventsParameter {
    fn example() -> Self {
        let mut filter_map: HashMap<String, Vec<String>> = HashMap::new();
        filter_map.insert(
            String::from("container"),
            vec![String::from("get_events_streaming_from_example")],
        );
        Self {
            filters: Some(filter_map),
            since: Some(String::from("3m")),
            stream: true,
            until: Some(String::from("10s")),
        }
    }
}

/// Internal representation of `EventsParameter` used for streaming events since filters can't be serialized in a single step.
/// It needs to be serialized to json, then to query. `TryInto` tries to perform the serialisation into json.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct EventsParameterStreamingQuery {
    pub filters: Option<String>,
    pub since: Option<String>,
    pub stream: bool,
    pub until: Option<String>,
}

impl TryFrom<EventsParameter> for EventsParameterStreamingQuery {
    type Error = PodtenderError;
    fn try_from(param: EventsParameter) -> Result<Self, Self::Error> {
        let filters = utils::convert_from_map_to_json_string(param.filters)?;
        Ok(EventsParameterStreamingQuery {
            filters,
            since: param.since,
            stream: true,
            until: param.until,
        })
    }
}
