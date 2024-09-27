// Based mainly on the docs at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM and https://chip8.gulrak.net/

mod core;
mod utils;
mod audio;

use error_iter::ErrorIter as _;
use log::error;
use std::sync::{Arc, Mutex};
use clap::Parser;
use pixels::{Pixels, SurfaceTexture};
use std::rc::Rc;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
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
    // Browser "arguments"
    // These are hardcoded for now until I create a more flexible web view
    let _rom = include_bytes!("../nyancat.ch8").to_vec();
    let clock = 20000;
    let target = core::Target::XO;
    let mut core = core::Chip8::new(target, clock, _rom, 48000);

    // Handle CLI arguments
    #[cfg(not(target_arch = "wasm32"))]
    {
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
        core = core::Chip8::new(_args.target, clock, _rom, 48000);
    }

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
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(core::WIDTH as f64, core::HEIGHT as f64);
        WindowBuilder::new()
            .with_title("chippy")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("WindowBuilder error")
    };

    let window = Rc::new(window);

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowExtWebSys;

        // Retrieve current width and height dimensions of browser client window
        let get_window_size = || {
            let client_window = web_sys::window().unwrap();
            LogicalSize::new(
                client_window.inner_width().unwrap().as_f64().unwrap(),
                client_window.inner_height().unwrap().as_f64().unwrap(),
            )
        };

        let window = Rc::clone(&window);

        // Initialize winit window with current dimensions of browser client
        window.set_inner_size(get_window_size());

        let client_window = web_sys::window().unwrap();

        // Attach winit canvas to body element
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");

        // Listen for resize event on browser client. Adjust winit window dimensions
        // on event trigger
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
            let size = get_window_size();
            window.set_inner_size(size)
        }) as Box<dyn FnMut(_)>);
        client_window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());
        Pixels::new_async(core::WIDTH as u32, core::HEIGHT as u32, surface_texture).await.expect("Pixels error")
    };

    let _res = event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let mut core = arc_parent.lock().unwrap();
            draw_core(&mut core, pixels.frame_mut());
            drop(core);
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
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

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn draw_core(core: &mut core::Chip8, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = core::WIDTH - 1 - (i % core::WIDTH >> !core.high_res as u8);
        let y = i / core::WIDTH >> !core.high_res as u8;

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
