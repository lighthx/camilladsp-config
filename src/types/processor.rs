use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CompressorParameters {
    pub channels: usize,
    #[serde(default)]
    pub monitor_channels: Option<Vec<usize>>,
    #[serde(default)]
    pub process_channels: Option<Vec<usize>>,
    pub attack: f64,
    pub release: f64,
    pub threshold: f64,
    pub factor: f64,
    #[serde(default)]
    pub makeup_gain: Option<f64>,
    #[serde(default)]
    pub soft_clip: Option<bool>,
    #[serde(default)]
    pub clip_limit: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct NoiseGateParameters {
    pub channels: usize,
    #[serde(default)]
    pub monitor_channels: Option<Vec<usize>>,
    #[serde(default)]
    pub process_channels: Option<Vec<usize>>,
    pub attack: f64,
    pub release: f64,
    pub threshold: f64,
    pub attenuation: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum Processor {
    Compressor {
        #[serde(default)]
        description: Option<String>,
        parameters: CompressorParameters,
    },
    NoiseGate {
        #[serde(default)]
        description: Option<String>,
        parameters: NoiseGateParameters,
    },
}
