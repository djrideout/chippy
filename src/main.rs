// Based mainly on the docs at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

use macroquad::prelude::*;
use clap::Parser;

// Command line arguments
#[derive(Parser, Debug)]
struct Args {
    // The path to the ROM to read
    #[arg(short, long)]
    input: String,
}

const WIDTH: usize = 64;
const WIDTH_BYTES: usize = WIDTH / 8;
const HEIGHT: usize = 32;
const SCALE: f32 = 10.0;

#[macroquad::main("chippy")]
async fn main() {
    // Load ROM file
    let _args = Args::parse();
    let _result = load_file(&_args.input).await;
    let _rom = match _result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the ROM: {error:?}")
    };

    // Registers
    let mut r_v: [u8; 16] = [0; 16]; // 16 general purpose "Vx" registers (x is 0-F)
    let mut r_i: u16 = 0; // Register "I"
    let mut r_pc: u16 = 0x200; // Program counter
    let mut r_sp: u8 = 0; // Stack pointer
    let mut r_delay: u8 = 0; // Delay timer
    let mut r_sound: u8 = 0; // Sound timer

    // Stack
    let mut stack: [u16; 16] = [0; 16];

    // Memory
    let mut mem: [u8; 0x1000] = [0; 0x1000];
    let mut i = 0x200; // ROM starts at 0x200 in memory
    for byte in _rom.into_iter() {
        mem[i] = byte;
        i += 1;
    }

    // Display (64x32 monochrome)
    let mut display: [u8; WIDTH_BYTES * HEIGHT] = [0; WIDTH_BYTES * HEIGHT];

    loop {
        //clear_background(RED);

        //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        //draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        // Get opcode
        let _op: u16 = ((mem[r_pc as usize] as u16) << 8) | mem[(r_pc + 1) as usize] as u16;
        r_pc += 2;

        // Decode opcode
        if _op == 0x00E0 {
            // 00E0 - CLS
            // Clear the display.
            for i in 0 .. WIDTH_BYTES * HEIGHT {
                display[i] = 0;
            }
        } else if (_op & !0xFFF) == 0x1000 {
            // 1nnn - JP addr
            // Jump to location nnn.
            r_pc = _op & 0xFFF;
        } else if (_op & !0xFFF) == 0x6000 {
            // 6xkk - LD Vx, byte
            // Set Vx = kk.
            let _x = ((_op & 0xF00) >> 8) as usize;
            r_v[_x] = (_op & 0xFF) as u8;
        } else if (_op & !0xFFF) == 0xA000 {
            // Annn - LD I, addr
            // Set I = nnn.
            r_i = _op & 0xFFF;
        } else if (_op & !0xFFF) == 0xD000 {
            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            // Sprites are 8 pixels (8 bits/1 byte) wide and from 1 to 15 pixels in height,
            // So each byte is one row of the sprite.
            // TODO: Support unaligned sprites
            let _x = (r_v[((_op & 0xF00) >> 8) as usize] / 8) as usize;
            let _y = r_v[((_op & 0xF0) >> 4) as usize] as usize;
            let _n = (_op & 0xF) as usize;
            let mut unset = false;
            for i in 0 .. _n {
                let byte = mem[i + r_i as usize];
                let display_index = (_y + i) * WIDTH_BYTES + _x;
                let prev = display[display_index];
                display[display_index] ^= byte;
                unset = unset || ((!display[display_index] & prev) > 0);
            }
            r_v[0xF] = unset as u8;
        }

        // Render display
        clear_background(BLACK);
        for i in 0 .. HEIGHT {
            for j in 0 .. WIDTH_BYTES {
                let byte = display[i * WIDTH_BYTES + j];
                for k in 0 .. u8::BITS {
                    let pixel = byte >> (7 - k) & 1;
                    if pixel == 1 {
                        draw_rectangle(SCALE * (j as u32 * u8::BITS + k) as f32, SCALE * i as f32, SCALE * 1.0, SCALE * 1.0, WHITE);
                    }
                }
            }
        }

        next_frame().await;
    }
}
