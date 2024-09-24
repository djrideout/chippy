// Based mainly on the docs at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM and https://chip8.gulrak.net/

mod core;
//mod utils;
mod audio;

use std::sync::{Arc, Mutex};
use clap::Parser;
use pixels::{Error, Pixels, SurfaceTexture};
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

// Constants
const SCALE: usize = 6;
// const PLANE_COLORS: [Color; 3] = [
//     BLACK, // Plane 0
//     GRAY, // Plane 1
//     LIGHTGRAY // Planes 0 and 1
// ];

struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

fn main() {
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
    let mut world = World::new();
    let res = event_loop.run(|event, elwt| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            world.draw(pixels.frame_mut());
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

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });


    // Handle arguments
    // let _args = Args::parse();
    // //let _rom = utils::load_rom(&_args.input).await;
    // let mut clock = _args.clock;
    // if clock == 0 {
    //     match _args.target {
    //         core::Target::Chip => clock = 11,
    //         core::Target::SuperModern => clock = 30,
    //         core::Target::SuperLegacy => clock = 30,
    //         core::Target::XO => clock = 1000
    //     }
    // }

    // request_new_screen_size((core::WIDTH * SCALE) as f32, (core::HEIGHT * SCALE) as f32);

    // let core = core::Chip8::new(_args.target, clock, _rom, 48000);

    // // Create Arc pointer to safely share the Chip8 core between the main thread and the audio thread
    // let arc_parent = Arc::new(Mutex::new(core));
    // let arc_child = arc_parent.clone();
    
    // let get_sample = move |i: usize| {
    //     // Lock the mutex while generating samples in the audio thread
    //     let mut core = arc_child.lock().unwrap();
    //     if i % 2 == 0 {
    //         while !core.run_inst() {}
    //     }
    //     core.get_sample()
    // };
    // let player = audio::AudioPlayer::new(48000, get_sample);
    // player.start();

    // loop {
    //     // Lock the mutex while handling inputs/rendering
    //     let mut core = arc_parent.lock().unwrap();

    //     // Handle key presses
    //     for i in 0 ..= 0xF as usize {
    //         core.prev_keys[i] = core.curr_keys[i];
    //         if is_key_released(KEYMAP[i]) {
    //             core.curr_keys[i] = false;
    //         } else if is_key_pressed(KEYMAP[i]) || is_key_down(KEYMAP[i]) {
    //             core.curr_keys[i] = true;
    //         } else {
    //             core.curr_keys[i] = false;
    //         }
    //     }

    //     // Render display
    //     let _true_scale = (SCALE << !core.high_res as u32) as f32;
    //     clear_background(WHITE);

    //     for i in 0 .. core::HEIGHT {
    //         let _both = core.buffer_planes[0][i] & core.buffer_planes[1][i];
    //         let _zero = core.buffer_planes[0][i] & !_both;
    //         let _one = core.buffer_planes[1][i] & !_both;
    //         for j in 0 .. core::WIDTH {
    //             if _zero & (1 << j) > 0 {
    //                 draw_rectangle(_true_scale * (core::WIDTH - 1 - j) as f32, _true_scale * i as f32, _true_scale, _true_scale, PLANE_COLORS[0]);
    //             }
    //             if _one & (1 << j) > 0 {
    //                 draw_rectangle(_true_scale * (core::WIDTH - 1 - j) as f32, _true_scale * i as f32, _true_scale, _true_scale, PLANE_COLORS[1]);
    //             }
    //             if _both & (1 << j) > 0 {
    //                 draw_rectangle(_true_scale * (core::WIDTH - 1 - j) as f32, _true_scale * i as f32, _true_scale, _true_scale, PLANE_COLORS[2]);
    //             }
    //         }
    //     }

    //     // Manually unlock the mutex while waiting for the next frame so the audio thread can drive the emulation
    //     drop(core);

    //     next_frame().await;
    // }
}

const BOX_SIZE: i16 = 64;

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > core::WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > core::HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % core::WIDTH as usize) as i16;
            let y = (i / core::WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
