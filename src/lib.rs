pub mod types;
pub use types::*;

impl Configuration {
    pub fn to_yaml_string(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }

    pub fn from_yaml_string(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    pub fn to_yaml_writer<W: std::io::Write>(&self, writer: W) -> Result<(), serde_yaml::Error> {
        serde_yaml::to_writer(writer, self)
    }

    pub fn from_yaml_reader<R: std::io::Read>(reader: R) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_reader(reader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_roundtrip_simpleconfig() {
        let yaml = r#"---
devices:
  samplerate: 44100
  chunksize: 1024
  target_level: 512
  enable_rate_adjust: true
  capture:
    type: Alsa
    channels: 2
    device: "hw:Loopback,0,1"
    format: S16LE
  playback:
    type: Alsa
    channels: 2
    device: "hw:Loopback,0,5"
    format: S32LE

filters:
  lowpass_fir:
    type: Conv
    parameters:
      type: Raw
      filename: filter.txt

mixers:
  monomix:
    channels:
      in: 2
      out: 2
    mapping:
      - dest: 0
        sources:
          - channel: 0
            gain: -6
            inverted: false
          - channel: 1
            gain: -6
            inverted: false
      - dest: 1
        sources:
          - channel: 0
            gain: -6
            inverted: false
          - channel: 1
            gain: -6
            inverted: false

pipeline:
  - type: Mixer
    name: monomix
  - type: Filter
    channels: [0, 1]
    names:
      - lowpass_fir
"#;
        let config = Configuration::from_yaml_string(yaml).expect("Failed to parse simpleconfig.yml");

        assert_eq!(config.devices.samplerate, 44100);
        assert_eq!(config.devices.chunksize, 1024);
        assert_eq!(config.devices.target_level, Some(512));
        assert_eq!(config.devices.enable_rate_adjust, Some(true));

        // Verify capture device
        match &config.devices.capture {
            CaptureDevice::Alsa { channels, device, format, .. } => {
                assert_eq!(*channels, 2);
                assert_eq!(device, "hw:Loopback,0,1");
                assert_eq!(*format, Some(SampleFormat::S16LE));
            }
            _ => panic!("Expected Alsa capture device"),
        }

        // Verify playback device
        match &config.devices.playback {
            PlaybackDevice::Alsa { channels, device, format, .. } => {
                assert_eq!(*channels, 2);
                assert_eq!(device, "hw:Loopback,0,5");
                assert_eq!(*format, Some(SampleFormat::S32LE));
            }
            _ => panic!("Expected Alsa playback device"),
        }

        // Verify filters
        let filters = config.filters.as_ref().expect("Expected filters");
        assert!(filters.contains_key("lowpass_fir"));

        // Verify mixers
        let mixers = config.mixers.as_ref().expect("Expected mixers");
        let monomix = mixers.get("monomix").expect("Expected monomix");
        assert_eq!(monomix.channels.r#in, 2);
        assert_eq!(monomix.channels.out, 2);

        // Verify pipeline
        let pipeline = config.pipeline.as_ref().expect("Expected pipeline");
        assert_eq!(pipeline.len(), 2);

        // Round-trip: serialize back and re-parse
        let yaml_out = config.to_yaml_string().expect("Failed to serialize");
        let config2 = Configuration::from_yaml_string(&yaml_out).expect("Failed to re-parse");
        assert_eq!(config, config2);
    }

    #[test]
    fn test_build_config_programmatically() {
        let config = Configuration {
            title: Some("Test Config".to_string()),
            description: None,
            devices: Devices {
                samplerate: 48000,
                chunksize: 1024,
                queuelimit: None,
                silence_threshold: None,
                silence_timeout: None,
                capture: CaptureDevice::CoreAudio(CaptureDeviceCA {
                    channels: 2,
                    device: None,
                    format: None,
                    labels: None,
                }),
                playback: PlaybackDevice::CoreAudio(PlaybackDeviceCA {
                    channels: 2,
                    device: None,
                    format: None,
                    exclusive: None,
                }),
                enable_rate_adjust: None,
                target_level: None,
                adjust_period: None,
                resampler: None,
                capture_samplerate: None,
                stop_on_rate_change: None,
                rate_measure_interval: None,
                volume_ramp_time: None,
                volume_limit: None,
                multithreaded: None,
                worker_threads: None,
            },
            mixers: None,
            filters: Some(HashMap::from([(
                "my_gain".to_string(),
                Filter::Gain {
                    description: Some("Volume adjustment".to_string()),
                    parameters: GainParameters {
                        gain: -3.0,
                        inverted: None,
                        mute: None,
                        scale: Some(GainScale::Decibel),
                    },
                },
            )])),
            processors: None,
            pipeline: Some(vec![PipelineStep::Filter(PipelineStepFilter {
                channels: Some(vec![0, 1]),
                names: vec!["my_gain".to_string()],
                description: None,
                bypassed: None,
            })]),
        };

        let yaml = config.to_yaml_string().expect("Failed to serialize");
        assert!(yaml.contains("CoreAudio"));
        assert!(yaml.contains("samplerate: 48000"));
        assert!(yaml.contains("my_gain"));

        // Verify it round-trips
        let config2 = Configuration::from_yaml_string(&yaml).expect("Failed to re-parse");
        assert_eq!(config, config2);
    }
}
