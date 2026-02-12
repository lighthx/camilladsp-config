use serde::{Deserialize, Serialize};

use super::common::GainScale;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MixerChannels {
    pub r#in: usize,
    pub out: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MixerSource {
    pub channel: usize,
    #[serde(default)]
    pub gain: Option<f64>,
    #[serde(default)]
    pub inverted: Option<bool>,
    #[serde(default)]
    pub mute: Option<bool>,
    #[serde(default)]
    pub scale: Option<GainScale>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct MixerMapping {
    pub dest: usize,
    pub sources: Vec<MixerSource>,
    #[serde(default)]
    pub mute: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Mixer {
    #[serde(default)]
    pub description: Option<String>,
    pub channels: MixerChannels,
    pub mapping: Vec<MixerMapping>,
    #[serde(default)]
    pub labels: Option<Vec<Option<String>>>,
}
