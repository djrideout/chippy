// The audio playback thread

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct AudioPlayer {
    output_stream: cpal::Stream,
}

impl AudioPlayer {
    pub fn new(frequency: u32) -> AudioPlayer {
        let host = cpal::default_host();
        let output_device = match host.default_output_device() {
            Some(device) => device,
            None => panic!("No audio device found")
        };
        let sample_rate = cpal::SampleRate(frequency);
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate,
            buffer_size: cpal::BufferSize::Default
        };
        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let len = data.len();
            for (i, sample) in data.iter_mut().enumerate() {
                *sample = f32::sin(i as f32 * 2.0 * 3.14159 / len as f32);
            }
        };
        let output_stream = match output_device.build_output_stream(&config, output_data_fn, Self::error, None) {
            Ok(stream) => stream,
            Err(err) => panic!("Error when building stream: {}", err)
        };
        return AudioPlayer {
            output_stream
        };
    }

    pub fn start(&self) {
        match self.output_stream.play() {
            Ok(_) => {},
            Err(err) => panic!("Stream play error: {}", err)
        };
    }

    fn error(err: cpal::StreamError) {
        panic!("AudioPlayer error: {}", err);
    }
}
