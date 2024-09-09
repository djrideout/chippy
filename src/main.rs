// Based mainly on the docs at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

use std::time::{Duration, Instant};
use ::rand::prelude::*;
use macroquad::prelude::*;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone, Default, PartialEq)]
enum Target {
    #[default]
    Chip,
    SuperModern,
    SuperLegacy,
    XO
}

// Command line arguments
#[derive(Parser, Debug)]
struct Args {
    // The path to the ROM to read
    #[arg(short, long)]
    input: String,

    // The number of instructions to run per frame
    #[arg(short, long, default_value_t = 8)]
    clock: u32,

    // The platform you are targetting
    #[arg(short, long, default_value_t, value_enum)]
    target: Target,
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
const WIDTH: usize = 128;
const HEIGHT: usize = 64;
const SCALE: f32 = 5.0;
const FRAME_DURATION: Duration = Duration::new(0, 16666666); // Approximately 60fps

#[macroquad::main("chippy")]
async fn main() {
    // Handle arguments
    let _args = Args::parse();
    if _args.clock == 0 {
        panic!("Clock rate must be positive");
    }
    let _result = load_file(&_args.input).await;
    let _rom = match _result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the ROM: {error:?}")
    };

    // General purpose registers
    let mut r_v: [u8; 16] = [0; 16]; // 16 general purpose "Vx" registers (x is 0-F)

    // Indexing registers
    let mut r_i: usize = 0; // Register "I"
    let mut r_pc: usize = 0x200; // Program counter
    let mut r_sp: usize = 0; // Stack pointer

    // Other registers
    let mut r_delay: u8 = 0; // Delay timer
    let mut r_sound: u8 = 0; // Sound timer

    // Stack
    let mut stack: [u16; 16] = [0; 16];

    // Memory
    let mut mem: [u8; 0x1000] = [0; 0x1000];
    // Load font into memory
    mem[0x00] = 0xF0; mem[0x01] = 0x90; mem[0x02] = 0x90; mem[0x03] = 0x90; mem[0x04] = 0xF0; // Digit 0
    mem[0x05] = 0x20; mem[0x06] = 0x60; mem[0x07] = 0x20; mem[0x08] = 0x20; mem[0x09] = 0x70; // Digit 1
    mem[0x0A] = 0xF0; mem[0x0B] = 0x10; mem[0x0C] = 0xF0; mem[0x0D] = 0x80; mem[0x0E] = 0xF0; // Digit 2
    mem[0x0F] = 0xF0; mem[0x10] = 0x10; mem[0x11] = 0xF0; mem[0x12] = 0x10; mem[0x13] = 0xF0; // Digit 3
    mem[0x14] = 0x90; mem[0x15] = 0x90; mem[0x16] = 0xF0; mem[0x17] = 0x10; mem[0x18] = 0x10; // Digit 4
    mem[0x19] = 0xF0; mem[0x1A] = 0x80; mem[0x1B] = 0xF0; mem[0x1C] = 0x10; mem[0x1D] = 0xF0; // Digit 5
    mem[0x1E] = 0xF0; mem[0x1F] = 0x80; mem[0x20] = 0xF0; mem[0x21] = 0x90; mem[0x22] = 0xF0; // Digit 6
    mem[0x23] = 0xF0; mem[0x24] = 0x10; mem[0x25] = 0x20; mem[0x26] = 0x40; mem[0x27] = 0x40; // Digit 7
    mem[0x28] = 0xF0; mem[0x29] = 0x90; mem[0x2A] = 0xF0; mem[0x2B] = 0x90; mem[0x2C] = 0xF0; // Digit 8
    mem[0x2D] = 0xF0; mem[0x2E] = 0x90; mem[0x2F] = 0xF0; mem[0x30] = 0x10; mem[0x31] = 0xF0; // Digit 9
    mem[0x32] = 0xF0; mem[0x33] = 0x90; mem[0x34] = 0xF0; mem[0x35] = 0x90; mem[0x36] = 0x90; // Digit A
    mem[0x37] = 0xE0; mem[0x38] = 0x90; mem[0x39] = 0xE0; mem[0x3A] = 0x90; mem[0x3B] = 0xE0; // Digit B
    mem[0x3C] = 0xF0; mem[0x3D] = 0x80; mem[0x3E] = 0x80; mem[0x3F] = 0x80; mem[0x40] = 0xF0; // Digit C
    mem[0x41] = 0xE0; mem[0x42] = 0x90; mem[0x43] = 0x90; mem[0x44] = 0x90; mem[0x45] = 0xE0; // Digit D
    mem[0x46] = 0xF0; mem[0x47] = 0x80; mem[0x48] = 0xF0; mem[0x49] = 0x80; mem[0x4A] = 0xF0; // Digit E
    mem[0x4B] = 0xF0; mem[0x4C] = 0x80; mem[0x4D] = 0xF0; mem[0x4E] = 0x80; mem[0x4F] = 0x80; // Digit F
    // Load ROM into memory
    let mut i = 0x200; // ROM starts at 0x200 in memory
    for byte in _rom.into_iter() {
        mem[i] = byte;
        i += 1;
    }


    // Halting flag (waiting for input)
    let mut halting = false;

    // Previous opcode, for halting purposes
    let mut prev_op = ((mem[r_pc] as u16) << 8) | mem[r_pc + 1] as u16;

    // Display (64x32 monochrome)
    let mut high_res = false; // For high-res resolution mode
    let mut display: [u128; HEIGHT] = [0; HEIGHT];
    request_new_screen_size(WIDTH as f32 * SCALE, HEIGHT as f32 * SCALE);

    // Key press states
    let mut prev_keys: [bool; 16] = [false; 16];
    let mut curr_keys: [bool; 16] = [false; 16];

    loop {
        // Time at the start of this frame
        let _t0 = Instant::now();

        // Handle key presses
        for i in 0 ..= 0xF as usize {
            prev_keys[i] = curr_keys[i];
            if is_key_released(KEYMAP[i]) {
                curr_keys[i] = false;
            } else if is_key_pressed(KEYMAP[i]) || is_key_down(KEYMAP[i]) {
                curr_keys[i] = true;
            } else {
                curr_keys[i] = false;
            }
        }

        // Remaining instructions to run for this frame
        let mut remaining = _args.clock;

        while remaining > 0 {
            // Get opcode
            let mut op: u16 = ((mem[r_pc] as u16) << 8) | mem[r_pc + 1] as u16;
            if halting {
                op = prev_op;
            } else {
                r_pc += 2;
            }

            // Decode opcode
            let _x = ((op & 0xF00) >> 8) as usize;
            let _y = ((op & 0xF0) >> 4) as usize;
            let _kk = (op & 0xFF) as u8;
            let _nnn = (op & 0xFFF) as usize;
            if op == 0x00E0 {
                // 00E0 - CLS
                // Clear the display.
                for i in 0 .. HEIGHT {
                    display[i] = 0;
                }
            } else if op == 0x00EE {
                // 00EE - RET
                // Return from a subroutine.
                r_sp -= 1;
                r_pc = stack[r_sp] as usize;
            } else if op == 0x00FE {
                // 00FE - LOW
                // Disable high-resolution mode.
                high_res = false;
            } else if op == 0x00FF {
                // 00FF - HIGH
                // Enable high-resolution mode.
                high_res = true;
            } else if (op & 0xF000) == 0x1000 {
                // 1nnn - JP addr
                // Jump to location nnn.
                r_pc = _nnn;
            } else if (op & 0xF000) == 0x2000 {
                // 2nnn - CALL addr
                // Call subroutine at nnn.
                stack[r_sp] = r_pc as u16;
                r_sp += 1;
                r_pc = _nnn;
            } else if (op & 0xF000) == 0x3000 {
                // 3xkk - SE Vx, byte
                // Skip next instruction if Vx = kk.
                if r_v[_x] == _kk {
                    r_pc += 2;
                }
            } else if (op & 0xF000) == 0x4000 {
                // 4xkk - SNE Vx, byte
                // Skip next instruction if Vx != kk.
                if r_v[_x] != _kk {
                    r_pc += 2;
                }
            } else if (op & 0xF000) == 0x5000 {
                // 5xy0 - SE Vx, Vy
                // Skip next instruction if Vx = Vy.
                if r_v[_x] == r_v[_y] {
                    r_pc += 2;
                }
            } else if (op & 0xF000) == 0x6000 {
                // 6xkk - LD Vx, byte
                // Set Vx = kk.
                r_v[_x] = _kk;
            } else if (op & 0xF000) == 0x7000 {
                // 7xkk - ADD Vx, byte
                // Set Vx = Vx + kk.
                let _next = r_v[_x] as u16 + _kk as u16;
                r_v[_x] = _next as u8;
            } else if (op & 0xF00F) == 0x8000 {
                // 8xy0 - LD Vx, Vy
                // Set Vx = Vy.
                r_v[_x] = r_v[_y];
            } else if (op & 0xF00F) == 0x8001 {
                // 8xy1 - OR Vx, Vy
                // Set Vx = Vx OR Vy.
                r_v[_x] |= r_v[_y];
                if _args.target == Target::Chip {
                    r_v[0xF] = 0;
                }
            } else if (op & 0xF00F) == 0x8002 {
                // 8xy2 - AND Vx, Vy
                // Set Vx = Vx AND Vy.
                r_v[_x] &= r_v[_y];
                if _args.target == Target::Chip {
                    r_v[0xF] = 0;
                }
            } else if (op & 0xF00F) == 0x8003 {
                // 8xy3 - XOR Vx, Vy
                // Set Vx = Vx XOR Vy.
                r_v[_x] ^= r_v[_y];
                if _args.target == Target::Chip {
                    r_v[0xF] = 0;
                }
            } else if (op & 0xF00F) == 0x8004 {
                // 8xy4 - ADD Vx, Vy
                // Set Vx = Vx + Vy, set VF = carry.
                let _next_x = r_v[_x] as u16 + r_v[_y] as u16;
                r_v[_x] = _next_x as u8;
                r_v[0xF] = (_next_x > 0xFF) as u8;
            } else if (op & 0xF00F) == 0x8005 {
                // 8xy5 - SUB Vx, Vy
                // Set Vx = Vx - Vy, set VF = NOT borrow.
                let _prev_x = r_v[_x];
                if _prev_x < r_v[_y] {
                    r_v[_x] = !(r_v[_y] - _prev_x - 1);
                } else {
                    r_v[_x] = _prev_x - r_v[_y];
                }
                r_v[0xF] = (_prev_x >= r_v[_y]) as u8;
            } else if (op & 0xF00F) == 0x8006 {
                // 8xy6 - SHR Vx {, Vy}
                // Set Vx = Vx SHR 1.
                let mut prev = r_v[_y];
                if _args.target == Target::SuperLegacy || _args.target == Target::SuperModern {
                    prev = r_v[_x];
                }
                r_v[_x] = prev >> 1;
                r_v[0xF] = prev & 1;
            } else if (op & 0xF00F) == 0x8007 {
                // 8xy7 - SUBN Vx, Vy
                // Set Vx = Vy - Vx, set VF = NOT borrow.
                let _prev_x = r_v[_x];
                if r_v[_y] < _prev_x {
                    r_v[_x] = !(_prev_x - r_v[_y] - 1);
                } else {
                    r_v[_x] = r_v[_y] - _prev_x;
                }
                r_v[0xF] = (r_v[_y] >= _prev_x) as u8;
            } else if (op & 0xF00F) == 0x800E {
                // 8xyE - SHL Vx {, Vy}
                // Set Vx = Vx SHL 1.
                let mut prev = r_v[_y];
                if _args.target == Target::SuperLegacy || _args.target == Target::SuperModern {
                    prev = r_v[_x];
                }
                r_v[_x] = prev << 1;
                r_v[0xF] = (prev & 0x80) >> 7;
            } else if (op & 0xF000) == 0x9000 {
                // 9xy0 - SNE Vx, Vy
                // Skip next instruction if Vx != Vy.
                if r_v[_x] != r_v[_y] {
                    r_pc += 2;
                }
            } else if (op & 0xF000) == 0xA000 {
                // Annn - LD I, addr
                // Set I = nnn.
                r_i = _nnn;
            } else if (op & 0xF000) == 0xB000 {
                // Bnnn - JP V0, addr
                // Jump to location nnn + V0.
                let mut loc = r_v[0] as usize;
                if _args.target == Target::SuperLegacy || _args.target == Target::SuperModern {
                    let _i = (_nnn & 0xF00) >> 8;
                    loc = r_v[_i] as usize;
                }
                r_pc = _nnn + loc;
            } else if (op & 0xF000) == 0xC000 {
                // Cxkk - RND Vx, byte
                // Set Vx = random byte AND kk.
                r_v[_x] = thread_rng().gen::<u8>() & _kk;
            } else if (op & 0xF000) == 0xD000 {
                // Dxyn - DRW Vx, Vy, nibble
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                // Sprites are 8 pixels (8 bits/1 byte) wide and from 1 to 15 pixels in height,
                // So each byte is one row of the sprite.
                let _x_mod = WIDTH >> !high_res as u8;
                let _y_mod = HEIGHT >> !high_res as u8;
                let _x_coord = r_v[_x] as usize % _x_mod;
                let _y_coord = r_v[_y] as usize % _y_mod;
                let _n = (op & 0xF) as usize;
                let mut unset = false;
                for i in 0 .. _n {
                    let _row_i = _y_coord + i;
                    if _row_i >= HEIGHT {
                        continue;
                    }
                    let _sprite_row = mem[i + r_i] as u128;
                    let _curr = display[_row_i];
                    let _shift = WIDTH - 1 - _x_coord;
                    if _shift < 7 {
                        display[_row_i] ^= _sprite_row >> (7 - _shift);
                    } else {
                        display[_row_i] ^= _sprite_row << (_shift - 7);
                    }
                    unset = unset || (!display[_row_i] & _curr) > 0;
                }
                r_v[0xF] = unset as u8;
            } else if (op & 0xF0FF) == 0xE09E {
                // Ex9E - SKP Vx
                // Skip next instruction if key with the value of Vx is pressed.
                if curr_keys[r_v[_x] as usize] {
                    r_pc += 2;
                }
            } else if (op & 0xF0FF) == 0xE0A1 {
                // ExA1 - SKNP Vx
                // Skip next instruction if key with the value of Vx is not pressed.
                if !curr_keys[r_v[_x] as usize] {
                    r_pc += 2;
                }
            } else if (op & 0xF0FF) == 0xF007 {
                // Fx07 - LD Vx, DT
                // Set Vx = delay timer value.
                r_v[_x] = r_delay;
            } else if (op & 0xF0FF) == 0xF00A {
                // Fx0A - LD Vx, K
                // Wait for a key press, store the value of the key in Vx.
                halting = true;
                // I guess I'll just grab the first key that releases between previous and current
                for i in 0 ..= 0xF as usize {
                    if prev_keys[i] && !curr_keys[i] {
                        halting = false;
                        r_v[_x] = i as u8;
                        break;
                    }
                }
            } else if (op & 0xF0FF) == 0xF015 {
                // Fx15 - LD DT, Vx
                // Set delay timer = Vx.
                r_delay = r_v[_x];
            } else if (op & 0xF0FF) == 0xF018 {
                // Fx18 - LD ST, Vx
                // Set sound timer = Vx.
                r_sound = r_v[_x];
            } else if (op & 0xF0FF) == 0xF01E {
                // Fx1E - ADD I, Vx
                // Set I = I + Vx.
                r_i += r_v[_x] as usize;
            } else if (op & 0xF0FF) == 0xF029 {
                // Fx29 - LD F, Vx
                // Set I = location of sprite for digit Vx.
                r_i = r_v[_x] as usize * 5;
            } else if (op & 0xF0FF) == 0xF033 {
                // Fx33 - LD B, Vx
                // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                mem[r_i] = r_v[_x] / 100 % 10;
                mem[r_i + 1] = r_v[_x] / 10 % 10;
                mem[r_i + 2] = r_v[_x] % 10;
            } else if (op & 0xF0FF) == 0xF055 {
                // Fx55 - LD [I], Vx
                // Store registers V0 through Vx in memory starting at location I.
                for i in 0 ..= _x {
                    mem[r_i + i] = r_v[i];
                }
                if _args.target == Target::Chip || _args.target == Target::XO {
                    r_i += _x + 1;
                }
            } else if (op & 0xF0FF) == 0xF065 {
                // Fx65 - LD Vx, [I]
                // Read registers V0 through Vx from memory starting at location I.
                for i in 0 ..= _x {
                    r_v[i] = mem[r_i + i];
                }
                if _args.target == Target::Chip || _args.target == Target::XO {
                    r_i += _x + 1;
                }
            } else {
                panic!("Unimplemented opcode 0x{:0x}", op);
            }

            prev_op = op;
            remaining -= 1;
        }

        // Decrement timers.
        // They decrement at 60hz, so because fps is around 60, just decrement once per frame.
        if r_delay > 0 {
            r_delay -= 1;
        }
        if r_sound > 0 {
            r_sound -= 1;
        }

        // Render display
        let _true_scale = SCALE * ((!high_res as u32) << 1) as f32;
        clear_background(BLACK);
        for i in 0 .. HEIGHT {
            let _row = display[i];
            for j in 0 .. WIDTH {
                if _row & (1 << j) > 0 {
                    draw_rectangle(_true_scale * (WIDTH - 1 - j) as f32, _true_scale * i as f32, _true_scale, _true_scale, WHITE);
                }
            }
        }

        next_frame().await;

        // Time at the end of this frame
        let _t1 = Instant::now();

        // If the frame length is too short, delay the remaining milliseconds to limit the fps
        let _delta = _t1.duration_since(_t0);
        if _delta < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - _delta);
        }
    }
}
