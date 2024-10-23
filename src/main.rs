mod utils;
mod core;

use basic_emu_frontend::{block_on, keymap::Keymap, Frontend, SyncModes, VirtualKeyCode};
use clap::Parser;
use std::sync::{Arc, Mutex};

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

    // The method for syncing the emulation
    #[arg(short, long, default_value_t, value_enum)]
    sync: SyncModes
}

// Keymap (Assumes QWERTY for now)
//     QWERTY:        CHIP-8;
//     1 2 3 4        1 2 3 C
//     Q W E R   ->   4 5 6 D
//     A S D F        7 8 9 E
//     Z X C V        A 0 B F
const KEYMAP: [VirtualKeyCode; 16] = [
    VirtualKeyCode::X,
    VirtualKeyCode::Key1,
    VirtualKeyCode::Key2,
    VirtualKeyCode::Key3,
    VirtualKeyCode::Q,
    VirtualKeyCode::W,
    VirtualKeyCode::E,
    VirtualKeyCode::A,
    VirtualKeyCode::S,
    VirtualKeyCode::D,
    VirtualKeyCode::Z,
    VirtualKeyCode::C,
    VirtualKeyCode::Key4,
    VirtualKeyCode::R,
    VirtualKeyCode::F,
    VirtualKeyCode::V
];

fn main() {
    // In the browser, create and run the core/frontend from the JS side.
    #[cfg(not(target_arch = "wasm32"))]
    block_on(run());
}

async fn run() {
    let core = {
        let _args = Args::parse();
        let _rom = utils::load_rom(&_args.input);
        let mut clock = _args.clock;
        if clock == 0 {
            clock = match _args.target {
                core::Target::Chip => 11,
                core::Target::SuperModern => 30,
                core::Target::SuperLegacy => 30,
                core::Target::XO => 1000
            }
        }
        core::Chip8::new(_args.target, clock, _rom)
    };

    let sync_mode = Args::parse().sync;

    let frontend = Frontend::new(
        Arc::new(Mutex::new(core)),
        Keymap::new(&KEYMAP),
        sync_mode
    );

    frontend.start().await
}
