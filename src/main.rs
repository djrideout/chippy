// Based mainly on the docs at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM and https://chip8.gulrak.net/

mod core;
mod utils;
mod audio;

use std::sync::{Arc, Mutex};
use clap::Parser;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

// Command line arguments
#[derive(Parser, Debug)]
struct Args {
    // The path to the ROM to read
    #[arg(short, long)]
    input: String,

    // The number of instructions to run per frame
    #[arg(short, long, default_value_t = 0)]
    clock: u32,

    // The platform you are targetting
    #[arg(short, long, default_value_t, value_enum)]
    target: core::Target,
}

// Keymap (Assumes QWERTY for now)
//     QWERTY:        CHIP-8;
//     1 2 3 4        1 2 3 C
//     Q W E R   ->   4 5 6 D
//     A S D F        7 8 9 E
//     Z X C V        A 0 B F
const KEYMAP: [KeyCode; 16] = [
    KeyCode::KeyX,
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::KeyQ,
    KeyCode::KeyW,
    KeyCode::KeyE,
    KeyCode::KeyA,
    KeyCode::KeyS,
    KeyCode::KeyD,
    KeyCode::KeyZ,
    KeyCode::KeyC,
    KeyCode::Digit4,
    KeyCode::KeyR,
    KeyCode::KeyF,
    KeyCode::KeyV
];

fn main() {
    // Handle arguments
    let _args = Args::parse();
    let _rom = utils::load_rom(&_args.input);
    let mut clock = _args.clock;
    if clock == 0 {
        match _args.target {
            core::Target::Chip => clock = 11,
            core::Target::SuperModern => clock = 30,
            core::Target::SuperLegacy => clock = 30,
            core::Target::XO => clock = 1000
        }
    }

    // Create core
    let core = core::Chip8::new(_args.target, clock, _rom, 48000);

    // Setup audio
    // Create Arc pointer to safely share the Chip8 core between the main thread and the audio thread
    let arc_parent = Arc::new(Mutex::new(core));
    let arc_child = arc_parent.clone();

    let get_sample = move |i: usize| {
        // Lock the mutex while generating samples in the audio thread
        let mut core = arc_child.lock().unwrap();
        if i % 2 == 0 {
            while !core.run_inst() {}
        }
        core.get_sample()
    };
    let player = audio::AudioPlayer::new(48000, get_sample);
    player.start();

    // Set up graphics buffer and window
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(core::WIDTH as f64, core::HEIGHT as f64);
        WindowBuilder::new()
            .with_title("chippy")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(core::WIDTH as u32, core::HEIGHT as u32, surface_texture).unwrap()
    };
    let res = event_loop.run(|event, elwt| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            let mut core = arc_parent.lock().unwrap();
            draw_core(&mut core, pixels.frame_mut());
            drop(core);
            if let Err(err) = pixels.render() {
                elwt.exit();
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    elwt.exit();
                    return;
                }
            }

            let mut core = arc_parent.lock().unwrap();
            // Handle key presses
            for i in 0 ..= 0xF as usize {
                core.prev_keys[i] = core.curr_keys[i];
                if input.key_released(KEYMAP[i]) {
                    core.curr_keys[i] = false;
                } else if input.key_pressed(KEYMAP[i]) || input.key_held(KEYMAP[i]) {
                    core.curr_keys[i] = true;
                } else {
                    core.curr_keys[i] = false;
                }
            }
            drop(core);

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}

fn draw_core(core: &mut core::Chip8, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = core::WIDTH - 1 - (i % core::WIDTH);
        let y = i / core::WIDTH;

        let _both = core.buffer_planes[0][y] & core.buffer_planes[1][y];
        let _zero = core.buffer_planes[0][y] & !_both;
        let _one = core.buffer_planes[1][y] & !_both;

        let rgba = if _both & (1 << x) > 0 {
            [0xd3, 0xd3, 0xd3, 0xff]
        } else if _zero & (1 << x) > 0 {
            [0x00, 0x00, 0x00, 0xff]
        } else if _one & (1 << x) > 0 {
            [0x80, 0x80, 0x80, 0xff]
        } else {
            [0xFF, 0xFF, 0xFF, 0xff]
        };

        pixel.copy_from_slice(&rgba);
    }
}
