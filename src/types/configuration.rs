use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::devices::Devices;
use super::filters::Filter;
use super::mixer::Mixer;
use super::pipeline::PipelineStep;
use super::processor::Processor;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Configuration {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub devices: Devices,
    #[serde(default)]
    pub mixers: Option<HashMap<String, Mixer>>,
    #[serde(default)]
    pub filters: Option<HashMap<String, Filter>>,
    #[serde(default)]
    pub processors: Option<HashMap<String, Processor>>,
    #[serde(default)]
    pub pipeline: Option<Vec<PipelineStep>>,
}
