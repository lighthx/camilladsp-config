use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
pub enum AsyncSincInterpolation {
    Nearest,
    Linear,
    Quadratic,
    Cubic,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
pub enum AsyncSincProfile {
    VeryFast,
    Fast,
    Balanced,
    Accurate,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub enum AsyncSincWindow {
    Hann,
    Hann2,
    Blackman,
    Blackman2,
    BlackmanHarris,
    BlackmanHarris2,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum AsyncSincParameters {
    Profile {
        profile: AsyncSincProfile,
    },
    Free {
        sinc_len: usize,
        interpolation: AsyncSincInterpolation,
        window: AsyncSincWindow,
        f_cutoff: Option<f32>,
        oversampling_factor: usize,
    },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, utoipa::ToSchema)]
pub enum AsyncPolyInterpolation {
    Linear,
    Cubic,
    Quintic,
    Septic,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum Resampler {
    AsyncPoly {
        interpolation: AsyncPolyInterpolation,
    },
    AsyncSinc(AsyncSincParameters),
    Synchronous,
}
