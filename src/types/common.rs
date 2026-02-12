use serde::{Deserialize, Serialize};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub enum SampleFormat {
    S16LE,
    S24LE,
    S24LE3,
    S32LE,
    FLOAT32LE,
    FLOAT64LE,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub enum FileFormat {
    TEXT,
    S16LE,
    S24LE,
    S24LE3,
    S32LE,
    FLOAT32LE,
    FLOAT64LE,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
pub enum GainScale {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "dB")]
    Decibel,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub enum TimeUnit {
    #[serde(rename = "ms")]
    Milliseconds,
    #[serde(rename = "mm")]
    Millimetres,
    #[serde(rename = "samples")]
    Samples,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
pub enum Signal {
    Sine { freq: f64, level: f64 },
    Square { freq: f64, level: f64 },
    WhiteNoise { level: f64 },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
pub enum VolumeFader {
    Aux1 = 1,
    Aux2 = 2,
    Aux3 = 3,
    Aux4 = 4,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
pub enum LoudnessFader {
    Main = 0,
    Aux1 = 1,
    Aux2 = 2,
    Aux3 = 3,
    Aux4 = 4,
}
