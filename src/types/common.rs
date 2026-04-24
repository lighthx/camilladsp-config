use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
#[allow(non_camel_case_types)]
pub enum FileSampleFormat {
    TEXT,
    S16_LE,
    S24_4_RJ_LE,
    S24_4_LJ_LE,
    S24_3_LE,
    S32_LE,
    F32_LE,
    F64_LE,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
#[allow(non_camel_case_types)]
pub enum BinarySampleFormat {
    S16_LE,
    S24_4_RJ_LE,
    S24_4_LJ_LE,
    S24_3_LE,
    S32_LE,
    F32_LE,
    F64_LE,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
#[allow(non_camel_case_types)]
pub enum AlsaSampleFormat {
    S16_LE,
    S24_3_LE,
    S24_4_LE,
    S32_LE,
    F32_LE,
    F64_LE,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub enum WasapiSampleFormat {
    S16,
    S24,
    S32,
    F32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
#[allow(non_camel_case_types)]
pub enum AsioSampleFormat {
    S16_LE,
    S24_4_LE,
    S24_3_LE,
    S32_LE,
    F32_LE,
    F64_LE,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub enum CoreAudioSampleFormat {
    S16,
    S24,
    S32,
    F32,
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
    #[serde(rename = "us")]
    Microseconds,
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
