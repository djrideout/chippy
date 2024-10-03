use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, StreamConfig};

pub struct AudioPlayer {
    output_stream: cpal::Stream,
    config: StreamConfig
}

impl AudioPlayer {
    pub fn new<F: 'static + Send + Fn() -> f32>(get_sample: F) -> AudioPlayer {
        let host = cpal::default_host();
        let output_device = match host.default_output_device() {
            Some(device) => device,
            None => panic!("No audio device found")
        };
        let supported_config = match output_device.default_output_config() {
            Ok(config) => config,
            Err(_err) => panic!("Default output config error: {}", _err)
        };
        let config = StreamConfig::from(supported_config);
        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for (_, sample) in data.iter_mut().enumerate() {
                *sample = get_sample();
            }
        };
        let output_stream = match output_device.build_output_stream(&config, output_data_fn, Self::error, None) {
            Ok(stream) => stream,
            Err(err) => panic!("Error when building stream: {}", err)
        };
        AudioPlayer {
            output_stream,
            config
        }
    }

    pub fn run(&self) {
        match self.output_stream.play() {
            Ok(_) => {},
            Err(err) => panic!("Stream play error: {}", err)
        };
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.config.sample_rate.0
    }

    pub fn get_num_channels(&self) -> usize {
        self.config.channels as usize
    }

    fn error(err: cpal::StreamError) {
        panic!("AudioPlayer error: {}", err);
    }
}
