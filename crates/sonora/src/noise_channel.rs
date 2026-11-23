//! Noise-only mono processing for Orvia's 48 kHz capture blocks.

use crate::{
    AudioProcessing, Config, StreamConfig,
    config::{NoiseSuppression, NoiseSuppressionLevel},
};

/// A 48 kHz mono channel with the high WebRTC noise-suppression preset.
#[derive(Debug)]
pub struct NoiseChannel(AudioProcessing);

impl Default for NoiseChannel {
    fn default() -> Self {
        let config = Config {
            noise_suppression: Some(NoiseSuppression {
                level: NoiseSuppressionLevel::High,
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut processor = AudioProcessing::builder()
            .config(config)
            .capture_config(StreamConfig::new(48_000, 1))
            .build();
        processor.inner.retain_mono_noise_capture();
        Self(processor)
    }
}

impl NoiseChannel {
    /// Process one complete 10 ms mono frame.
    pub fn process(&mut self, input: &[f32; 480], output: &mut [f32; 480]) {
        let cfg = self.0.capture_config;
        self.0
            .inner
            .process_stream(&[input], &cfg, &cfg, &mut [output]);
    }

    /// Copy the complete state of an uninterrupted stream at a frame boundary.
    /// Both channels use the same fixed configuration and retain their allocated buffers.
    pub fn split_from(&mut self, source: &Self) {
        self.0.inner.copy_noise_history_from(&source.0.inner);
    }
}
