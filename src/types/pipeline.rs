use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PipelineStepMixer {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub bypassed: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PipelineStepFilter {
    #[serde(default)]
    pub channels: Option<Vec<usize>>,
    pub names: Vec<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub bypassed: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PipelineStepProcessor {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub bypassed: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum PipelineStep {
    Mixer(PipelineStepMixer),
    Filter(PipelineStepFilter),
    Processor(PipelineStepProcessor),
}
