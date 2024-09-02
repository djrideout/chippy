// Based mainly on the docs at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

use macroquad::prelude::*;

#[macroquad::main("chippy")]
async fn main() {
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

    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await;
    }
}
