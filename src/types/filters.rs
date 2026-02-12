use serde::{Deserialize, Serialize};

use super::common::{FileFormat, GainScale, LoudnessFader, TimeUnit, VolumeFader};

// --- Convolution parameters ---

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ConvParametersRaw {
    pub filename: String,
    #[serde(default)]
    pub format: Option<FileFormat>,
    #[serde(default)]
    pub skip_bytes_lines: Option<usize>,
    #[serde(default)]
    pub read_bytes_lines: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ConvParametersWav {
    pub filename: String,
    #[serde(default)]
    pub channel: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum ConvParameters {
    Raw(ConvParametersRaw),
    Wav(ConvParametersWav),
    Values { values: Vec<f64> },
    Dummy { length: usize },
}

// --- Biquad parameter helper types ---

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ShelfSteepness {
    Q { freq: f64, q: f64, gain: f64 },
    Slope { freq: f64, slope: f64, gain: f64 },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PeakingWidth {
    Q { freq: f64, q: f64, gain: f64 },
    Bandwidth { freq: f64, bandwidth: f64, gain: f64 },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum NotchWidth {
    Q { freq: f64, q: f64 },
    Bandwidth { freq: f64, bandwidth: f64 },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GeneralNotchParams {
    pub freq_p: f64,
    pub freq_z: f64,
    pub q_p: f64,
    #[serde(default)]
    pub normalize_at_dc: Option<bool>,
}

// --- Biquad parameters ---

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum BiquadParameters {
    Free {
        a1: f64,
        a2: f64,
        b0: f64,
        b1: f64,
        b2: f64,
    },
    Highpass {
        freq: f64,
        q: f64,
    },
    Lowpass {
        freq: f64,
        q: f64,
    },
    Peaking(PeakingWidth),
    Highshelf(ShelfSteepness),
    HighshelfFO {
        freq: f64,
        gain: f64,
    },
    Lowshelf(ShelfSteepness),
    LowshelfFO {
        freq: f64,
        gain: f64,
    },
    HighpassFO {
        freq: f64,
    },
    LowpassFO {
        freq: f64,
    },
    Allpass(NotchWidth),
    AllpassFO {
        freq: f64,
    },
    Bandpass(NotchWidth),
    Notch(NotchWidth),
    GeneralNotch(GeneralNotchParams),
    LinkwitzTransform {
        freq_act: f64,
        q_act: f64,
        freq_target: f64,
        q_target: f64,
    },
}

// --- BiquadCombo parameters ---

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GraphicEqualizerParameters {
    #[serde(default)]
    pub freq_min: Option<f32>,
    #[serde(default)]
    pub freq_max: Option<f32>,
    pub gains: Vec<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum BiquadComboParameters {
    LinkwitzRileyHighpass { freq: f64, order: usize },
    LinkwitzRileyLowpass { freq: f64, order: usize },
    ButterworthHighpass { freq: f64, order: usize },
    ButterworthLowpass { freq: f64, order: usize },
    Tilt { gain: f64 },
    FivePointPeq {
        fls: f64,
        qls: f64,
        gls: f64,
        fp1: f64,
        qp1: f64,
        gp1: f64,
        fp2: f64,
        qp2: f64,
        gp2: f64,
        fp3: f64,
        qp3: f64,
        gp3: f64,
        fhs: f64,
        qhs: f64,
        ghs: f64,
    },
    GraphicEqualizer(GraphicEqualizerParameters),
}

// --- Other filter parameter types ---

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GainParameters {
    pub gain: f64,
    #[serde(default)]
    pub inverted: Option<bool>,
    #[serde(default)]
    pub mute: Option<bool>,
    #[serde(default)]
    pub scale: Option<GainScale>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VolumeParameters {
    #[serde(default)]
    pub ramp_time: Option<f32>,
    pub fader: VolumeFader,
    pub limit: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LoudnessParameters {
    pub reference_level: f32,
    #[serde(default)]
    pub high_boost: Option<f32>,
    #[serde(default)]
    pub low_boost: Option<f32>,
    #[serde(default)]
    pub fader: Option<LoudnessFader>,
    #[serde(default)]
    pub attenuate_mid: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DelayParameters {
    pub delay: f64,
    #[serde(default)]
    pub unit: Option<TimeUnit>,
    #[serde(default)]
    pub subsample: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum DitherParameters {
    None { bits: usize },
    Flat { bits: usize, amplitude: f64 },
    Highpass { bits: usize },
    Fweighted441 { bits: usize },
    FweightedLong441 { bits: usize },
    FweightedShort441 { bits: usize },
    Gesemann441 { bits: usize },
    Gesemann48 { bits: usize },
    Lipshitz441 { bits: usize },
    LipshitzLong441 { bits: usize },
    Shibata441 { bits: usize },
    ShibataHigh441 { bits: usize },
    ShibataLow441 { bits: usize },
    Shibata48 { bits: usize },
    ShibataHigh48 { bits: usize },
    ShibataLow48 { bits: usize },
    Shibata882 { bits: usize },
    ShibataLow882 { bits: usize },
    Shibata96 { bits: usize },
    ShibataLow96 { bits: usize },
    Shibata192 { bits: usize },
    ShibataLow192 { bits: usize },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DiffEqParameters {
    #[serde(default)]
    pub a: Option<Vec<f64>>,
    #[serde(default)]
    pub b: Option<Vec<f64>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LimiterParameters {
    #[serde(default)]
    pub soft_clip: Option<bool>,
    #[serde(default)]
    pub clip_limit: f64,
}

// --- Top-level Filter enum ---

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum Filter {
    Conv {
        #[serde(default)]
        description: Option<String>,
        parameters: ConvParameters,
    },
    Biquad {
        #[serde(default)]
        description: Option<String>,
        parameters: BiquadParameters,
    },
    BiquadCombo {
        #[serde(default)]
        description: Option<String>,
        parameters: BiquadComboParameters,
    },
    Delay {
        #[serde(default)]
        description: Option<String>,
        parameters: DelayParameters,
    },
    Gain {
        #[serde(default)]
        description: Option<String>,
        parameters: GainParameters,
    },
    Volume {
        #[serde(default)]
        description: Option<String>,
        parameters: VolumeParameters,
    },
    Loudness {
        #[serde(default)]
        description: Option<String>,
        parameters: LoudnessParameters,
    },
    Dither {
        #[serde(default)]
        description: Option<String>,
        parameters: DitherParameters,
    },
    DiffEq {
        #[serde(default)]
        description: Option<String>,
        parameters: DiffEqParameters,
    },
    Limiter {
        #[serde(default)]
        description: Option<String>,
        parameters: LimiterParameters,
    },
}
