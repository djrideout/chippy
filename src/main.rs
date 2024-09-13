// Based mainly on the docs at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM and https://chip8.gulrak.net/

use std::time::{Duration, Instant};
use ::rand::prelude::*;
use macroquad::prelude::*;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone, Default, PartialEq)]
enum Target {
    Chip, // This is "chip-8" in Gulrak's opcode table
    #[default]
    SuperModern, // This is "schipc" in Gulrak's opcode table
    SuperLegacy, // This is "schip-1.1" in Gulrak's opcode table
    XO // This is "xo-chip" in Gulrak's opcode table
}

// Command line arguments
#[derive(Parser, Debug)]
struct Args {
    // The path to the ROM to read
    #[arg(short, long)]
    input: String,

    // The number of instructions to run per frame
    #[arg(short, long, default_value_t = 16)]
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

// The CHIP-8 hexidecimal font
const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

// Constants
const WIDTH: usize = 128;
const HEIGHT: usize = 64;
const SCALE: usize = 6;
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
    mem[..FONT_SET.len()].copy_from_slice(&FONT_SET);
    // Load ROM into memory
    let mut i = 0x200; // ROM starts at 0x200 in memory
    for byte in _rom.into_iter() {
        mem[i] = byte;
        i += 1;
    }

    // Halting flag (waiting for input/drawing)
    let mut halting = false;

    // Previous opcode, for halting purposes
    let mut prev_op = ((mem[r_pc] as u16) << 8) | mem[r_pc + 1] as u16;

    // Display (64x32 monochrome)
    let mut high_res = false; // For high-res resolution mode
    let mut display: [u128; HEIGHT] = [0; HEIGHT];
    request_new_screen_size((WIDTH * SCALE) as f32, (HEIGHT * SCALE) as f32);

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
            remaining -= 1;

            // Get opcode
            let mut op = ((mem[r_pc] as u16) << 8) | mem[r_pc + 1] as u16;
            if halting {
                op = prev_op;
            } else {
                r_pc += 2;
            }

            // F000 is a 4-byte instruction, so if we need to skip an instruction and PC is on F000,
            // we should skip 4 bytes instead of 2.
            let _next_op = ((mem[r_pc] as u16) << 8) | mem[r_pc + 1] as u16;
            let mut skip_count: usize = 2;
            if _next_op == 0xF000 {
                skip_count = 4;
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
            } else if op == 0x00FD && _args.target != Target::Chip {
                // 00FD - EXIT
                // Exit the interpreter.
                return;
            } else if op == 0x00FE && _args.target != Target::Chip {
                // 00FE - LOW
                // Disable high-resolution mode.
                high_res = false;
                if _args.target != Target::SuperLegacy {
                    for i in 0 .. HEIGHT {
                        display[i] = 0;
                    }
                }
            } else if op == 0x00FF && _args.target != Target::Chip {
                // 00FF - HIGH
                // Enable high-resolution mode.
                high_res = true;
                if _args.target != Target::SuperLegacy {
                    for i in 0 .. HEIGHT {
                        display[i] = 0;
                    }
                }
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
                    r_pc += skip_count;
                }
            } else if (op & 0xF000) == 0x4000 {
                // 4xkk - SNE Vx, byte
                // Skip next instruction if Vx != kk.
                if r_v[_x] != _kk {
                    r_pc += skip_count;
                }
            } else if (op & 0xF00F) == 0x5000 {
                // 5xy0 - SE Vx, Vy
                // Skip next instruction if Vx = Vy.
                if r_v[_x] == r_v[_y] {
                    r_pc += skip_count;
                }
            } else if (op & 0xF00F) == 0x5002 && _args.target == Target::XO {
                // 5xy2
                // Write registers vX to vY to memory pointed to by I.
                for _i in _x ..= _y {
                    mem[r_i + _i - _x] = r_v[_i];
                }
            } else if (op & 0xF00F) == 0x5003 && _args.target == Target::XO {
                // 5xy3
                // Load registers vX to vY from memory pointed to by I.
                for _i in _x ..= _y {
                    r_v[_i] = mem[r_i + _i - _x];
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
                    r_pc += skip_count;
                }
            } else if (op & 0xF000) == 0xA000 {
                // Annn - LD I, addr
                // Set I = nnn.
                r_i = _nnn;
            } else if (op & 0xF000) == 0xB000 {
                // Bnnn - JP V0, addr / Bxnn - JP Vx, addr
                // Jump to location nnn + V0, or xnn + Vx on super.
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
                // Dxyn - DRW Vx, Vy, nibble / Dxy0 - DRW Vx, Vy, 0
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                // Sprites are 8 pixels (8 bits/1 byte) wide and from 1 to 15 pixels in height,
                // So each byte is one row of the sprite.
                // Draws a 16x16 sprite if n=0 and platform is not CHIP-8. (8x16 on super legacy in low-res mode)
                halting = true;
                if remaining == 0
                    || _args.target == Target::SuperModern
                    || _args.target == Target::SuperLegacy && high_res
                    || _args.target == Target::XO {
                    halting = false;
                    let _x_mod = WIDTH >> !high_res as u8;
                    let _y_mod = HEIGHT >> !high_res as u8;
                    let _x_coord = r_v[_x] as usize % _x_mod;
                    let _y_coord = r_v[_y] as usize % _y_mod;
                    let mut sprite_height = (op & 0xF) as usize;
                    let mut sprite_width = 8 as usize;
                    if _args.target != Target::Chip && sprite_height == 0 {
                        sprite_height = 16;
                        if _args.target != Target::SuperLegacy || high_res {
                            sprite_width = 16;
                        }
                    }
                    let mut unset = false;
                    for i in 0 .. sprite_height {
                        let mut row_i = _y_coord + i;
                        if row_i >= _y_mod {
                            if _args.target != Target::XO {
                                continue;
                            }
                            row_i %= _y_mod;
                        }
                        let mut sprite_row = mem[(sprite_width >> 3) * i + r_i] as u128;
                        if sprite_width == 16 {
                            sprite_row = (sprite_row << 8) | mem[(sprite_width >> 3) * i + r_i + 1] as u128;
                        }
                        let _curr = display[row_i];
                        let _shift = WIDTH - 1 - _x_coord;
                        if _shift < sprite_width - 1 {
                            display[row_i] ^= sprite_row >> (sprite_width - 1 - _shift);
                        } else {
                            display[row_i] ^= sprite_row << (_shift - sprite_width + 1);
                        }
                        if _args.target == Target::XO && _x_coord > _x_mod - sprite_width {
                            display[row_i] ^= sprite_row.rotate_right((_x_coord - (_x_mod - sprite_width)) as u32) & (!0u16 as u128) << 112;
                        }
                        if !high_res {
                            display[row_i] &= (!0u64 as u128) << 64;
                        }
                        unset = unset || (!display[row_i] & _curr) > 0;
                    }
                    r_v[0xF] = unset as u8;
                }
            } else if (op & 0xF0FF) == 0xE09E {
                // Ex9E - SKP Vx
                // Skip next instruction if key with the value of Vx is pressed.
                if curr_keys[r_v[_x] as usize] {
                    r_pc += skip_count;
                }
            } else if (op & 0xF0FF) == 0xE0A1 {
                // ExA1 - SKNP Vx
                // Skip next instruction if key with the value of Vx is not pressed.
                if !curr_keys[r_v[_x] as usize] {
                    r_pc += skip_count;
                }
            } else if op == 0xF000 && _args.target == Target::XO {
                // F000
                // Assign next 16 bit word to I, and set PC behind it. This is a four byte instruction.
                r_i = ((mem[r_pc] as usize) << 8) | mem[r_pc + 1] as usize;
                r_pc += 2;
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
        let _true_scale = (SCALE << !high_res as u32) as f32;
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
