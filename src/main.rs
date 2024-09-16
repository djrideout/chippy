// Based mainly on the docs at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM and https://chip8.gulrak.net/

#[cfg(test)]
mod test;

mod core;
mod utils;
mod audio;

use macroquad::prelude::*;
use clap::Parser;

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
    KeyCode::X,
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Q,
    KeyCode::W,
    KeyCode::E,
    KeyCode::A,
    KeyCode::S,
    KeyCode::D,
    KeyCode::Z,
    KeyCode::C,
    KeyCode::Key4,
    KeyCode::R,
    KeyCode::F,
    KeyCode::V
];

// Constants
const SCALE: usize = 6;
const PLANE_COLORS: [Color; 3] = [
    BLACK, // Plane 0
    GRAY, // Plane 1
    LIGHTGRAY // Planes 0 and 1
];

#[macroquad::main("chippy")]
async fn main() {
    let player = audio::AudioPlayer::new(48000);
    player.start();

    // Handle arguments
    let _args = Args::parse();
    let _rom = utils::load_rom(&_args.input).await;
    let mut clock = _args.clock;
    if clock == 0 {
        match _args.target {
            core::Target::Chip => clock = 11,
            core::Target::SuperModern => clock = 30,
            core::Target::SuperLegacy => clock = 30,
            core::Target::XO => clock = 1000
        }
    }

    request_new_screen_size((core::WIDTH * SCALE) as f32, (core::HEIGHT * SCALE) as f32);

    let mut chip8 = core::Chip8::new(_args.target, clock, _rom);

    loop {
        // Handle key presses
        for i in 0 ..= 0xF as usize {
            chip8.prev_keys[i] = chip8.curr_keys[i];
            if is_key_released(KEYMAP[i]) {
                chip8.curr_keys[i] = false;
            } else if is_key_pressed(KEYMAP[i]) || is_key_down(KEYMAP[i]) {
                chip8.curr_keys[i] = true;
            } else {
                chip8.curr_keys[i] = false;
            }
        }

        chip8.run_frame();

        // Render display
        let _true_scale = (SCALE << !chip8.high_res as u32) as f32;
        clear_background(WHITE);

        for i in 0 .. core::HEIGHT {
            let _both = chip8.planes[0][i] & chip8.planes[1][i];
            let _zero = chip8.planes[0][i] & !_both;
            let _one = chip8.planes[1][i] & !_both;
            for j in 0 .. core::WIDTH {
                if _zero & (1 << j) > 0 {
                    draw_rectangle(_true_scale * (core::WIDTH - 1 - j) as f32, _true_scale * i as f32, _true_scale, _true_scale, PLANE_COLORS[0]);
                }
                if _one & (1 << j) > 0 {
                    draw_rectangle(_true_scale * (core::WIDTH - 1 - j) as f32, _true_scale * i as f32, _true_scale, _true_scale, PLANE_COLORS[1]);
                }
                if _both & (1 << j) > 0 {
                    draw_rectangle(_true_scale * (core::WIDTH - 1 - j) as f32, _true_scale * i as f32, _true_scale, _true_scale, PLANE_COLORS[2]);
                }
            }
        }

        next_frame().await;
    }
}
