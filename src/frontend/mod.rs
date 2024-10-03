mod audio;
mod display;

use std::sync::{Arc, Mutex};
use winit::event::VirtualKeyCode;

#[derive(PartialEq, Clone, Copy)]
pub enum SyncModes {
    VSync,
    AudioCallback,
    AudioBuffer
}

pub trait Core: Send + 'static {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn get_sample_queue_length(&self) -> usize;
    fn is_internal_mono(&self) -> bool;
    fn draw(&self, frame: &mut [u8]);
    fn set_seconds_per_output_sample(&mut self, value: f32);
    fn set_num_output_channels(&mut self, value: usize);
    fn press_key(&mut self, key_index: usize);
    fn release_key(&mut self, key_index: usize);
    fn run_inst(&mut self) -> bool;
    fn run_frame(&mut self);
    fn get_sample(&mut self) -> f32;
}

pub struct Frontend<const N: usize> {
    display: display::Display<N>,
    audio_player: audio::AudioPlayer
}

impl<const N: usize> Frontend<N> {
    pub fn new(core: impl Core, keymap: [VirtualKeyCode; N], sync_mode: SyncModes) -> Frontend<N> {
        // Create Arcs to share the core between the audio and rendering threads
        let arc_parent = Arc::new(Mutex::new(core));
        let arc_child = arc_parent.clone();

        // The get_sample callback is what drives the emulation
        // core.run_inst() will return true when enough instructions have run for a new sample to be ready
        let get_sample = move |i: usize| {
            // Lock the mutex while generating samples in the audio thread
            let mut core = arc_child.lock().unwrap();
            if sync_mode == SyncModes::AudioCallback && (!core.is_internal_mono() || i % 2 == 0) {
                while !core.run_inst() {}
            }
            core.get_sample()
        };
        let audio_player = audio::AudioPlayer::new(48000, get_sample);

        let arc_temp = arc_parent.clone();
        let mut core_temp = arc_temp.lock().unwrap();
        core_temp.set_seconds_per_output_sample(1.0 / 48000.0);
        core_temp.set_num_output_channels(2);
        drop(core_temp);

        let display = display::Display::new(arc_parent, keymap, sync_mode);

        Frontend {
            display,
            audio_player
        }
    }

    pub async fn start(&self) {
        self.audio_player.run();
        self.display.run().await
    }
}
