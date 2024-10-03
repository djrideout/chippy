mod utils;
mod frontend;
mod core;

use crate::frontend::SyncModes;
use clap::Parser;
use winit::event::VirtualKeyCode;

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
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Trace).expect("error initializing logger");
        wasm_bindgen_futures::spawn_local(run());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(run());
    }
}

async fn run() {
    // Browser arguments are hardcoded for now until I create a more flexible web view
    #[cfg(target_arch = "wasm32")]
    let core = {
        let _rom = include_bytes!("../nyancat.ch8").to_vec();
        let clock = 20000;
        let target = core::Target::XO;
        core::Chip8::new(target, clock, _rom)
    };

    #[cfg(not(target_arch = "wasm32"))]
    let core = {
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
        core::Chip8::new(_args.target, clock, _rom)
    };

    let frontend = frontend::Frontend::new(core, KEYMAP, Args::parse().sync);
    frontend.start().await
}
