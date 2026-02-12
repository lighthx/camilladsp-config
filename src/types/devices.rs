use serde::{Deserialize, Serialize};

use super::common::{SampleFormat, Signal};
use super::resampler::Resampler;

// --- Capture device sub-structs ---

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CaptureDeviceRawFile {
    pub channels: usize,
    pub filename: String,
    pub format: SampleFormat,
    #[serde(default)]
    pub extra_samples: Option<usize>,
    #[serde(default)]
    pub skip_bytes: Option<usize>,
    #[serde(default)]
    pub read_bytes: Option<usize>,
    #[serde(default)]
    pub labels: Option<Vec<Option<String>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CaptureDeviceWavFile {
    pub filename: String,
    #[serde(default)]
    pub extra_samples: Option<usize>,
    #[serde(default)]
    pub labels: Option<Vec<Option<String>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CaptureDeviceStdin {
    pub channels: usize,
    pub format: SampleFormat,
    #[serde(default)]
    pub extra_samples: Option<usize>,
    #[serde(default)]
    pub skip_bytes: Option<usize>,
    #[serde(default)]
    pub read_bytes: Option<usize>,
    #[serde(default)]
    pub labels: Option<Vec<Option<String>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CaptureDeviceBluez {
    #[serde(default)]
    pub service: Option<String>,
    pub dbus_path: String,
    pub format: SampleFormat,
    pub channels: usize,
    #[serde(default)]
    pub labels: Option<Vec<Option<String>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CaptureDeviceWasapi {
    pub channels: usize,
    pub device: Option<String>,
    pub format: SampleFormat,
    #[serde(default)]
    pub exclusive: Option<bool>,
    #[serde(default)]
    pub loopback: Option<bool>,
    #[serde(default)]
    pub labels: Option<Vec<Option<String>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CaptureDeviceCA {
    pub channels: usize,
    pub device: Option<String>,
    #[serde(default)]
    pub format: Option<SampleFormat>,
    #[serde(default)]
    pub labels: Option<Vec<Option<String>>>,
}

// --- CaptureDevice enum ---

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
pub enum CaptureDevice {
    #[serde(alias = "ALSA", alias = "alsa")]
    Alsa {
        channels: usize,
        device: String,
        #[serde(default)]
        format: Option<SampleFormat>,
        #[serde(default)]
        stop_on_inactive: Option<bool>,
        #[serde(default)]
        link_volume_control: Option<String>,
        #[serde(default)]
        link_mute_control: Option<String>,
        #[serde(default)]
        labels: Option<Vec<Option<String>>>,
    },
    #[serde(alias = "BLUEZ", alias = "bluez")]
    Bluez(CaptureDeviceBluez),
    #[serde(alias = "PULSE", alias = "pulse")]
    Pulse {
        channels: usize,
        device: String,
        format: SampleFormat,
        #[serde(default)]
        labels: Option<Vec<Option<String>>>,
    },
    RawFile(CaptureDeviceRawFile),
    WavFile(CaptureDeviceWavFile),
    #[serde(alias = "STDIN", alias = "stdin")]
    Stdin(CaptureDeviceStdin),
    #[serde(alias = "COREAUDIO", alias = "coreaudio")]
    CoreAudio(CaptureDeviceCA),
    #[serde(alias = "WASAPI", alias = "wasapi")]
    Wasapi(CaptureDeviceWasapi),
    #[serde(alias = "JACK", alias = "jack")]
    Jack {
        channels: usize,
        device: String,
        #[serde(default)]
        labels: Option<Vec<Option<String>>>,
    },
    SignalGenerator {
        channels: usize,
        signal: Signal,
        #[serde(default)]
        labels: Option<Vec<Option<String>>>,
    },
}

// --- Playback device sub-structs ---

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PlaybackDeviceWasapi {
    pub channels: usize,
    pub device: Option<String>,
    pub format: SampleFormat,
    #[serde(default)]
    pub exclusive: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PlaybackDeviceCA {
    pub channels: usize,
    pub device: Option<String>,
    #[serde(default)]
    pub format: Option<SampleFormat>,
    #[serde(default)]
    pub exclusive: Option<bool>,
}

// --- PlaybackDevice enum ---

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
pub enum PlaybackDevice {
    #[serde(alias = "ALSA", alias = "alsa")]
    Alsa {
        channels: usize,
        device: String,
        #[serde(default)]
        format: Option<SampleFormat>,
    },
    #[serde(alias = "PULSE", alias = "pulse")]
    Pulse {
        channels: usize,
        device: String,
        format: SampleFormat,
    },
    #[serde(alias = "FILE", alias = "file")]
    File {
        channels: usize,
        filename: String,
        format: SampleFormat,
        #[serde(default)]
        wav_header: Option<bool>,
    },
    #[serde(alias = "STDOUT", alias = "stdout")]
    Stdout {
        channels: usize,
        format: SampleFormat,
        #[serde(default)]
        wav_header: Option<bool>,
    },
    #[serde(alias = "COREAUDIO", alias = "coreaudio")]
    CoreAudio(PlaybackDeviceCA),
    #[serde(alias = "WASAPI", alias = "wasapi")]
    Wasapi(PlaybackDeviceWasapi),
    #[serde(alias = "JACK", alias = "jack")]
    Jack {
        channels: usize,
        device: String,
    },
}

// --- Devices struct ---

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Devices {
    pub samplerate: usize,
    pub chunksize: usize,
    #[serde(default)]
    pub queuelimit: Option<usize>,
    #[serde(default)]
    pub silence_threshold: Option<f64>,
    #[serde(default)]
    pub silence_timeout: Option<f64>,
    pub capture: CaptureDevice,
    pub playback: PlaybackDevice,
    #[serde(default)]
    pub enable_rate_adjust: Option<bool>,
    #[serde(default)]
    pub target_level: Option<usize>,
    #[serde(default)]
    pub adjust_period: Option<f32>,
    #[serde(default)]
    pub resampler: Option<Resampler>,
    #[serde(default)]
    pub capture_samplerate: Option<usize>,
    #[serde(default)]
    pub stop_on_rate_change: Option<bool>,
    #[serde(default)]
    pub rate_measure_interval: Option<f32>,
    #[serde(default)]
    pub volume_ramp_time: Option<f32>,
    #[serde(default)]
    pub volume_limit: Option<f32>,
    #[serde(default)]
    pub multithreaded: Option<bool>,
    #[serde(default)]
    pub worker_threads: Option<usize>,
}
