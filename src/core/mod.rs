// Based mainly on the docs at http://devernay.free.fr/hacks/chip8/C8TECH10.HTM and https://chip8.gulrak.net/

use wasm_bindgen::prelude::*;
use clap::ValueEnum;
use std::hash::{RandomState, BuildHasher, Hasher, DefaultHasher};
use basic_emu_frontend::{Core, Frontend, keymap::Keymap, SyncModes};
use std::collections::VecDeque;

#[cfg(test)]
mod test;

#[wasm_bindgen]
#[derive(ValueEnum, Debug, Clone, Default, PartialEq)]
pub enum Target {
    Chip, // This is "chip-8" in Gulrak's opcode table
    SuperModern, // This is "schipc" in Gulrak's opcode table
    SuperLegacy, // This is "schip-1.1" in Gulrak's opcode table
    #[default]
    XO // This is "xo-chip" in Gulrak's opcode table
}

// The 5-byte hexadecimal font
const SMALL_FONT_SET: [u8; 80] = [
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

// The 10-byte hexadecimal font from Octo
const BIG_FONT_SET: [u8; 160] = [
    0xFF, 0xFF, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, // 0
    0x18, 0x78, 0x78, 0x18, 0x18, 0x18, 0x18, 0x18, 0xFF, 0xFF, // 1
    0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // 2
    0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 3
    0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0x03, 0x03, // 4
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 5
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 6
    0xFF, 0xFF, 0x03, 0x03, 0x06, 0x0C, 0x18, 0x18, 0x18, 0x18, // 7
    0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 8
    0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 9
    0x7E, 0xFF, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xC3, // A
    0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, // B
    0x3C, 0xFF, 0xC3, 0xC0, 0xC0, 0xC0, 0xC0, 0xC3, 0xFF, 0x3C, // C
    0xFC, 0xFE, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFE, 0xFC, // D
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // E
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xC0, 0xC0, // F
];

// Constants
const FRAME_RATE: f32 = 60.0;
const WIDTH: usize = 128;
const HEIGHT: usize = 64;
const PLANE_COUNT: usize = 2;

#[wasm_bindgen]
pub struct Chip8 {
    // Public members
    // For high-res resolution mode
    high_res: bool,
    // Planes ready for rendering
    buffer_planes: [[u128; HEIGHT]; PLANE_COUNT],
    // Key press states
    prev_keys: [bool; 16],
    curr_keys: [bool; 16],

    // Private members
    // The target platform
    target: Target,
    // Instructions per second
    clock: u32,
    // Remaining cycles for a frame
    remaining: u32,
    // General purpose registers
    r_v: [u8; 16], // 16 general purpose "Vx" registers (x is 0-F)
    // Indexing registers
    r_i: usize, // Register "I"
    r_pc: usize, // Program counter
    r_sp: usize, // Stack pointer
    // Other registers
    r_delay: u8, // Delay timer
    r_audio: u8, // Audio timer
    // Stack
    stack: [u16; 16],
    // Memory
    mem: [u8; 0x10000], // only XO-CHIP officially supports 0x10000, the rest have 0x1000 but just use the full range for simplicity
    // Halting flag (waiting for input/drawing)
    halting: bool,
    // Previous opcode, for halting purposes
    prev_op: u16,
    // Display (128x64, 2 planes)
    enabled_planes: u8, // Flags for which of the 2 planes to draw on. If the bit is set, draw on the plane.
    active_planes: [[u128; HEIGHT]; PLANE_COUNT],
    // Audio
    num_output_channels: usize,
    seconds_per_output_sample: f32,
    seconds_per_instruction: f32,
    audio_time: f32,
    audio_buffer: u128,
    audio_frequency: f32,
    audio_oscillator: f32,
    sample_queue: VecDeque<f32>,
    // For the rando instruction
    rand_hasher: DefaultHasher
}

#[wasm_bindgen]
impl Chip8 {
    #[wasm_bindgen(constructor)]
    pub fn new(target: Target, clock: u32, rom: Vec<u8>) -> Chip8 {
        let mut chip8 = Chip8 {
            target,
            clock,
            remaining: clock,
            r_v: [0; 16],
            r_i: 0,
            r_pc: 0x200,
            r_sp: 0,
            r_delay: 0,
            r_audio: 0,
            stack: [0; 16],
            mem: [0; 0x10000],
            halting: false,
            prev_op: 0,
            enabled_planes: 0b01,
            high_res: false,
            active_planes: [
                [0; HEIGHT],
                [0; HEIGHT]
            ],
            buffer_planes: [
                [0; HEIGHT],
                [0; HEIGHT]
            ],
            prev_keys: [false; 16],
            curr_keys: [false; 16],
            num_output_channels: 0, // This is set by the frontend before emulation starts
            seconds_per_output_sample: 0.0, // This is set by the frontend before emulation starts
            seconds_per_instruction: 1.0 / (FRAME_RATE * clock as f32),
            audio_time: 0.0,
            audio_buffer: 0x0000FFFF0000FFFF0000FFFF0000FFFF, // Arbitrary pattern for non-XO buzzer
            audio_frequency: 4000.0,
            audio_oscillator: 0.0,
            sample_queue: VecDeque::new(),
            rand_hasher: RandomState::new().build_hasher()
        };

        // Load fonts into memory
        chip8.mem[..SMALL_FONT_SET.len()].copy_from_slice(&SMALL_FONT_SET); // 5-byte font
        chip8.mem[SMALL_FONT_SET.len()..SMALL_FONT_SET.len() + BIG_FONT_SET.len()].copy_from_slice(&BIG_FONT_SET); // 10-byte font

        // Load ROM into memory
        let mut i = 0x200; // ROM starts at 0x200 in memory
        for byte in rom.into_iter() {
            chip8.mem[i] = byte;
            i += 1;
        }

        chip8
    }

    #[wasm_bindgen]
    pub fn get_width(&self) -> usize {
        WIDTH
    }

    #[wasm_bindgen]
    pub fn get_height(&self) -> usize {
        HEIGHT
    }
}

impl Core for Chip8 {
    fn get_width(&self) -> usize {
        self.get_width()
    }

    fn get_height(&self) -> usize {
        self.get_height()
    }

    fn set_num_output_channels(&mut self, value: usize) {
        self.num_output_channels = value;
    }

    fn set_seconds_per_output_sample(&mut self, value: f32) {
        self.seconds_per_output_sample = value;
    }

    fn run_inst(&mut self) {
        self.remaining -= 1;

        // Get opcode
        let mut op = ((self.mem[self.r_pc] as u16) << 8) | self.mem[self.r_pc + 1] as u16;
        if self.halting {
            op = self.prev_op;
        } else {
            self.r_pc += 2;
        }

        // F000 is a 4-byte instruction, so if we need to skip an instruction and PC is on F000,
        // we should skip 4 bytes instead of 2.
        let _next_op = ((self.mem[self.r_pc] as u16) << 8) | self.mem[self.r_pc + 1] as u16;
        let mut skip_count: usize = 2;
        if _next_op == 0xF000 {
            skip_count = 4;
        }

        // Decode opcode
        let _n = (op & 0xF) as usize;
        let _x = ((op & 0xF00) >> 8) as usize;
        let _y = ((op & 0xF0) >> 4) as usize;
        let _kk = (op & 0xFF) as u8;
        let _nnn = (op & 0xFFF) as usize;

        'opcodes: {
            // 0-nibble param opcodes
            let mut opcode_matched = true;
            match op & 0xFFFF {
                0x00E0 => {
                    // 00E0 - CLS
                    // Clear the display.
                    for p in 0 .. PLANE_COUNT {
                        if (self.enabled_planes >> p) & 1 == 0 {
                            continue;
                        }
                        for i in 0 .. HEIGHT {
                            self.active_planes[p][i] = 0;
                        }
                    }
                }
                0x00EE => {
                    // 00EE - RET
                    // Return from a subroutine.
                    self.r_sp -= 1;
                    self.r_pc = self.stack[self.r_sp] as usize;
                }
                0x00FB if self.target != Target::Chip => {
                    // 00FB
                    // Scroll screen content right four pixels, in XO-CHIP only selected bit planes are scrolled
                    for p in 0 .. PLANE_COUNT {
                        if (self.enabled_planes >> p) & 1 == 0 {
                            continue;
                        }
                        for i in 0 .. HEIGHT {
                            self.active_planes[p][i] = self.active_planes[p][i] >> 4;
                        }
                    }
                }
                0x00FC if self.target != Target::Chip => {
                    // 00FC
                    // Scroll screen content left four pixels, in XO-CHIP only selected bit planes are scrolled
                    for p in 0 .. PLANE_COUNT {
                        if (self.enabled_planes >> p) & 1 == 0 {
                            continue;
                        }
                        for i in 0 .. HEIGHT {
                            self.active_planes[p][i] = self.active_planes[p][i] << 4;
                        }
                    }
                }
                0x00FD if self.target != Target::Chip => {
                    // 00FD - EXIT
                    // Exit the interpreter.
                    // This used to return from main, but I'll just leave it for now.
                }
                0x00FE if self.target != Target::Chip => {
                    // 00FE - LOW
                    // Disable high-resolution mode.
                    self.high_res = false;
                    if self.target != Target::SuperLegacy {
                        for p in 0 .. PLANE_COUNT {
                            for i in 0 .. HEIGHT {
                                self.active_planes[p][i] = 0;
                            }
                        }
                    }
                }
                0x00FF if self.target != Target::Chip => {
                    // 00FF - HIGH
                    // Enable high-resolution mode.
                    self.high_res = true;
                    if self.target != Target::SuperLegacy {
                        for p in 0 .. PLANE_COUNT {
                            for i in 0 .. HEIGHT {
                                self.active_planes[p][i] = 0;
                            }
                        }
                    }
                }
                0xF000 if self.target == Target::XO => {
                    // F000
                    // Assign next 16 bit word to I, and set PC behind it. This is a four byte instruction.
                    self.r_i = ((self.mem[self.r_pc] as usize) << 8) | self.mem[self.r_pc + 1] as usize;
                    self.r_pc += 2;
                }
                0xF002 if self.target == Target::XO => {
                    // F002
                    // Load 16 bytes audio pattern pointed to by I into audio pattern buffer
                    let mut new_buffer = 0_u128;
                    for _i in 0 .. 16 {
                        new_buffer = (new_buffer << 8) | self.mem[self.r_i + _i] as u128;
                    }
                    self.audio_buffer = new_buffer;
                }
                _ => opcode_matched = false
            }
            if opcode_matched {
                break 'opcodes;
            }

            // 1-nibble param opcodes, but the param is the least significant nibble
            opcode_matched = true;
            match op & 0xFFF0 {
                0x00C0 if self.target != Target::Chip => {
                    // 00Cn
                    // Scroll screen content down N pixels, in XO-CHIP only selected bit planes are scrolled
                    let _count = _n >> (self.target == Target::SuperLegacy && !self.high_res) as u8;
                    for p in 0 .. PLANE_COUNT {
                        if (self.enabled_planes >> p) & 1 == 0 {
                            continue;
                        }
                        for i in (0 ..= HEIGHT - 1).rev() {
                            if i < _count {
                                self.active_planes[p][i] = 0;
                                continue;
                            }
                            self.active_planes[p][i] = self.active_planes[p][i - _count];
                        }
                    }
                }
                0x00D0 if self.target == Target::XO => {
                    // 00Dn
                    // Scroll screen content up N hires pixel, in XO-CHIP only selected planes are scrolled
                    for p in 0 .. PLANE_COUNT {
                        if (self.enabled_planes >> p) & 1 == 0 {
                            continue;
                        }
                        for i in 0 ..= HEIGHT - 1 {
                            if i + _n >= HEIGHT {
                                self.active_planes[p][i] = 0;
                                continue;
                            }
                            self.active_planes[p][i] = self.active_planes[p][i + _n];
                        }
                    }
                }
                _ => opcode_matched = false
            }
            if opcode_matched {
                break 'opcodes;
            }

            // 1-nibble param opcodes
            opcode_matched = true;
            match op & 0xF0FF {
                0xE09E => {
                    // Ex9E - SKP Vx
                    // Skip next instruction if key with the value of Vx is pressed.
                    if self.curr_keys[self.r_v[_x] as usize] {
                        self.r_pc += skip_count;
                    }
                }
                0xE0A1 => {
                    // ExA1 - SKNP Vx
                    // Skip next instruction if key with the value of Vx is not pressed.
                    if !self.curr_keys[self.r_v[_x] as usize] {
                        self.r_pc += skip_count;
                    }
                }
                0xF001 if self.target == Target::XO => {
                    // Fx01
                    // Select bit planes to draw on to x (not vX) when drawing with Dxy0/Dxyn
                    self.enabled_planes = (_x & 0b11) as u8;
                }
                0xF007 => {
                    // Fx07 - LD Vx, DT
                    // Set Vx = delay timer value.
                    self.r_v[_x] = self.r_delay;
                }
                0xF00A => {
                    // Fx0A - LD Vx, K
                    // Wait for a key press, store the value of the key in Vx.
                    self.halting = true;
                    // I guess I'll just grab the first key that releases between previous and current
                    for i in 0 ..= 0xFusize {
                        if self.prev_keys[i] && !self.curr_keys[i] {
                            self.halting = false;
                            self.r_v[_x] = i as u8;
                            break;
                        }
                    }
                }
                0xF015 => {
                    // Fx15 - LD DT, Vx
                    // Set delay timer = Vx.
                    self.r_delay = self.r_v[_x];
                }
                0xF018 => {
                    // Fx18 - LD ST, Vx
                    // Set sound timer = Vx.
                    self.r_audio = self.r_v[_x];
                }
                0xF01E => {
                    // Fx1E - ADD I, Vx
                    // Set I = I + Vx.
                    self.r_i += self.r_v[_x] as usize;
                }
                0xF029 => {
                    // Fx29 - LD F, Vx
                    // Set I = location of 5-line sprite for digit Vx.
                    self.r_i = (self.r_v[_x] & 0xF) as usize * 5;
                }
                0xF030 if self.target != Target::Chip => {
                    // Fx30
                    // Set I = location of 10-line sprite for digit Vx.
                    self.r_i = SMALL_FONT_SET.len() + (self.r_v[_x] & 0xF) as usize * 10;
                }
                0xF033 => {
                    // Fx33 - LD B, Vx
                    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                    self.mem[self.r_i] = self.r_v[_x] / 100 % 10;
                    self.mem[self.r_i + 1] = self.r_v[_x] / 10 % 10;
                    self.mem[self.r_i + 2] = self.r_v[_x] % 10;
                }
                0xF03A if self.target == Target::XO => {
                    // Fx3A
                    // Set audio frequency for a audio pattern playback rate of 4000*2^((vX-64)/48)Hz
                    self.audio_frequency = 4000.0 * 2_f32.powf((self.r_v[_x] as f32 - 64.0) / 48.0);
                }
                0xF055 => {
                    // Fx55 - LD [I], Vx
                    // Store registers V0 through Vx in memory starting at location I.
                    for i in 0 ..= _x {
                        self.mem[self.r_i + i] = self.r_v[i];
                    }
                    if self.target == Target::Chip || self.target == Target::XO {
                        self.r_i += _x + 1;
                    }
                }
                0xF065 => {
                    // Fx65 - LD Vx, [I]
                    // Read registers V0 through Vx from memory starting at location I.
                    for i in 0 ..= _x {
                        self.r_v[i] = self.mem[self.r_i + i];
                    }
                    if self.target == Target::Chip || self.target == Target::XO {
                        self.r_i += _x + 1;
                    }
                }
                _ => opcode_matched = false
            }
            if opcode_matched {
                break 'opcodes;
            }

            // 2-nibble param opcodes
            opcode_matched = true;
            match op & 0xF00F {
                0x5000 => {
                    // 5xy0 - SE Vx, Vy
                    // Skip next instruction if Vx = Vy.
                    if self.r_v[_x] == self.r_v[_y] {
                        self.r_pc += skip_count;
                    }
                }
                0x5002 if self.target == Target::XO => {
                    // 5xy2
                    // Write registers vX to vY to memory pointed to by I.
                    for _i in _x ..= _y {
                        self.mem[self.r_i + _i - _x] = self.r_v[_i];
                    }
                }
                0x5003 if self.target == Target::XO => {
                    // 5xy3
                    // Load registers vX to vY from memory pointed to by I.
                    for _i in _x ..= _y {
                        self.r_v[_i] = self.mem[self.r_i + _i - _x];
                    }
                }
                0x8000 => {
                    // 8xy0 - LD Vx, Vy
                    // Set Vx = Vy.
                    self.r_v[_x] = self.r_v[_y];
                }
                0x8001 => {
                    // 8xy1 - OR Vx, Vy
                    // Set Vx = Vx OR Vy.
                    self.r_v[_x] |= self.r_v[_y];
                    if self.target == Target::Chip {
                        self.r_v[0xF] = 0;
                    }
                }
                0x8002 => {
                    // 8xy2 - AND Vx, Vy
                    // Set Vx = Vx AND Vy.
                    self.r_v[_x] &= self.r_v[_y];
                    if self.target == Target::Chip {
                        self.r_v[0xF] = 0;
                    }
                }
                0x8003 => {
                    // 8xy3 - XOR Vx, Vy
                    // Set Vx = Vx XOR Vy.
                    self.r_v[_x] ^= self.r_v[_y];
                    if self.target == Target::Chip {
                        self.r_v[0xF] = 0;
                    }
                }
                0x8004 => {
                    // 8xy4 - ADD Vx, Vy
                    // Set Vx = Vx + Vy, set VF = carry.
                    let _next_x = self.r_v[_x] as u16 + self.r_v[_y] as u16;
                    self.r_v[_x] = _next_x as u8;
                    self.r_v[0xF] = (_next_x > 0xFF) as u8;
                }
                0x8005 => {
                    // 8xy5 - SUB Vx, Vy
                    // Set Vx = Vx - Vy, set VF = NOT borrow.
                    let _prev_x = self.r_v[_x];
                    if _prev_x < self.r_v[_y] {
                        self.r_v[_x] = !(self.r_v[_y] - _prev_x - 1);
                    } else {
                        self.r_v[_x] = _prev_x - self.r_v[_y];
                    }
                    self.r_v[0xF] = (_prev_x >= self.r_v[_y]) as u8;
                }
                0x8006 => {
                    // 8xy6 - SHR Vx {, Vy}
                    // Set Vx = Vx SHR 1.
                    let mut prev = self.r_v[_y];
                    if self.target == Target::SuperLegacy || self.target == Target::SuperModern {
                        prev = self.r_v[_x];
                    }
                    self.r_v[_x] = prev >> 1;
                    self.r_v[0xF] = prev & 1;
                }
                0x8007 => {
                    // 8xy7 - SUBN Vx, Vy
                    // Set Vx = Vy - Vx, set VF = NOT borrow.
                    let _prev_x = self.r_v[_x];
                    if self.r_v[_y] < _prev_x {
                        self.r_v[_x] = !(_prev_x - self.r_v[_y] - 1);
                    } else {
                        self.r_v[_x] = self.r_v[_y] - _prev_x;
                    }
                    self.r_v[0xF] = (self.r_v[_y] >= _prev_x) as u8;
                }
                0x800E => {
                    // 8xyE - SHL Vx {, Vy}
                    // Set Vx = Vx SHL 1.
                    let mut prev = self.r_v[_y];
                    if self.target == Target::SuperLegacy || self.target == Target::SuperModern {
                        prev = self.r_v[_x];
                    }
                    self.r_v[_x] = prev << 1;
                    self.r_v[0xF] = (prev & 0x80) >> 7;
                }
                _ => opcode_matched = false
            }
            if opcode_matched {
                break 'opcodes;
            }

            //3-nibble param opcodes
            match op & 0xF000 {
                0x1000 => {
                    // 1nnn - JP addr
                    // Jump to location nnn.
                    self.r_pc = _nnn;
                }
                0x2000 => {
                    // 2nnn - CALL addr
                    // Call subroutine at nnn.
                    self.stack[self.r_sp] = self.r_pc as u16;
                    self.r_sp += 1;
                    self.r_pc = _nnn;
                }
                0x3000 => {
                    // 3xkk - SE Vx, byte
                    // Skip next instruction if Vx = kk.
                    if self.r_v[_x] == _kk {
                        self.r_pc += skip_count;
                    }
                }
                0x4000 => {
                    // 4xkk - SNE Vx, byte
                    // Skip next instruction if Vx != kk.
                    if self.r_v[_x] != _kk {
                        self.r_pc += skip_count;
                    }
                }
                0x6000 => {
                    // 6xkk - LD Vx, byte
                    // Set Vx = kk.
                    self.r_v[_x] = _kk;
                }
                0x7000 => {
                    // 7xkk - ADD Vx, byte
                    // Set Vx = Vx + kk.
                    let _next = self.r_v[_x] as u16 + _kk as u16;
                    self.r_v[_x] = _next as u8;
                }
                0x9000 => {
                    // 9xy0 - SNE Vx, Vy
                    // Skip next instruction if Vx != Vy.
                    if self.r_v[_x] != self.r_v[_y] {
                        self.r_pc += skip_count;
                    }
                }
                0xA000 => {
                    // Annn - LD I, addr
                    // Set I = nnn.
                    self.r_i = _nnn;
                }
                0xB000 => {
                    // Bnnn - JP V0, addr / Bxnn - JP Vx, addr
                    // Jump to location nnn + V0, or xnn + Vx on super.
                    let mut loc = self.r_v[0] as usize;
                    if self.target == Target::SuperLegacy || self.target == Target::SuperModern {
                        let _i = (_nnn & 0xF00) >> 8;
                        loc = self.r_v[_i] as usize;
                    }
                    self.r_pc = _nnn + loc;
                }
                0xC000 => {
                    // Cxkk - RND Vx, byte
                    // Set Vx = random byte AND kk.
                    self.rand_hasher.write_u8(self.r_v[_x]);
                    self.r_v[_x] = self.rand_hasher.finish() as u8 & _kk;
                }
                0xD000 => {
                    // Dxyn - DRW Vx, Vy, nibble / Dxy0 - DRW Vx, Vy, 0
                    // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                    // Sprites are 8 pixels (8 bits/1 byte) wide and from 1 to 15 pixels in height,
                    // So each byte is one row of the sprite.
                    // Draws a 16x16 sprite if n=0 and platform is not CHIP-8. (8x16 on super legacy in low-res mode)
                    self.halting = true;
                    if self.remaining == 0
                        || self.target == Target::SuperModern
                        || self.target == Target::SuperLegacy && self.high_res
                        || self.target == Target::XO {
                        self.halting = false;
                        let mut plane_offset = -1;
                        for p in 0 .. PLANE_COUNT {
                            if (self.enabled_planes >> p) & 1 == 0 {
                                continue;
                            }
                            plane_offset += 1;
                            let _x_mod = WIDTH >> !self.high_res as u8;
                            let _y_mod = HEIGHT >> !self.high_res as u8;
                            let _x_coord = self.r_v[_x] as usize % _x_mod;
                            let _y_coord = self.r_v[_y] as usize % _y_mod;
                            let mut sprite_height = (op & 0xF) as usize;
                            let mut sprite_width = 8 as usize;
                            if self.target != Target::Chip && sprite_height == 0 {
                                sprite_height = 16;
                                if self.target != Target::SuperLegacy || self.high_res {
                                    sprite_width = 16;
                                }
                            }
                            let mut unset = false;
                            for i in 0 .. sprite_height {
                                let mut row_i = _y_coord + i;
                                if row_i >= _y_mod {
                                    if self.target != Target::XO {
                                        continue;
                                    }
                                    row_i %= _y_mod;
                                }
                                // plane_offset will be non-negative at this point, so casting to usize is fine
                                let _base_addr = (sprite_width >> 3) * plane_offset as usize * sprite_height + self.r_i + i;
                                let mut sprite_row = self.mem[_base_addr] as u128;
                                if sprite_width == 16 {
                                    sprite_row = (sprite_row << 8) | self.mem[_base_addr + 1] as u128;
                                }
                                let _curr = self.active_planes[p][row_i];
                                let _shift = WIDTH - 1 - _x_coord;
                                if _shift < sprite_width - 1 {
                                    self.active_planes[p][row_i] ^= sprite_row >> (sprite_width - 1 - _shift);
                                } else {
                                    self.active_planes[p][row_i] ^= sprite_row << (_shift - (sprite_width - 1));
                                }
                                if self.target == Target::XO && _x_coord > _x_mod - sprite_width {
                                    self.active_planes[p][row_i] ^= sprite_row.rotate_right((_x_coord - (_x_mod - sprite_width)) as u32) & !0u128 << 112;
                                }
                                if !self.high_res {
                                    self.active_planes[p][row_i] &= !0u128 << 64;
                                }
                                unset = unset || (!self.active_planes[p][row_i] & _curr) > 0;
                            }
                            self.r_v[0xF] = unset as u8;
                        }
                    }
                }
                _ => panic!("Unimplemented opcode 0x{:0x}", op)
            }
        }

        self.prev_op = op;

        self.audio_time += self.seconds_per_instruction;
        if self.audio_time >= self.seconds_per_output_sample {
            self.audio_time -= self.seconds_per_output_sample;
            self.audio_oscillator = (self.audio_oscillator + self.seconds_per_output_sample * self.audio_frequency) % 128.0;
            for _i in 0..self.num_output_channels {
                self.sample_queue.push_back(match self.r_audio {
                    0 => 0.0,
                    _ => ((self.audio_buffer >> self.audio_oscillator as u32) & 1) as f32
                });
            }
        }

        if self.remaining == 0 {
            // This frame is over, reset the remaining instruction count
            self.remaining = self.clock;

            // Decrement timers.
            // They decrement at 60hz, so because fps is around 60, just decrement once per frame.
            if self.r_delay > 0 {
                self.r_delay -= 1;
            }
            if self.r_audio > 0 {
                self.r_audio -= 1;
            }
            if self.r_audio == 0 {
                self.audio_oscillator = 0.0;
            }

            // Copy the active planes over to the buffer planes
            for _p in 0 .. PLANE_COUNT {
                for _i in 0 .. HEIGHT {
                    self.buffer_planes[_p][_i] = self.active_planes[_p][_i];
                }
            }
        }
    }

    fn run_frame(&mut self) {
        loop {
            self.run_inst();
            if self.remaining == self.clock {
                break;
            }
        }
    }

    fn get_sample_queue_length(&self) -> usize {
        self.sample_queue.len()
    }

    fn get_sample(&mut self) -> f32 {
        match self.sample_queue.pop_front() {
            Some(sample) => sample,
            None => 0.0
        }
    }

    fn press_key(&mut self, key_index: usize) {
        self.prev_keys[key_index] = self.curr_keys[key_index];
        self.curr_keys[key_index] = true;
    }

    fn release_key(&mut self, key_index: usize) {
        self.prev_keys[key_index] = self.curr_keys[key_index];
        self.curr_keys[key_index] = false;
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = WIDTH - 1 - (i % WIDTH >> !self.high_res as u8);
            let y = i / WIDTH >> !self.high_res as u8;
    
            let _both = self.buffer_planes[0][y] & self.buffer_planes[1][y];
            let _zero = self.buffer_planes[0][y] & !_both;
            let _one = self.buffer_planes[1][y] & !_both;
    
            let rgba = if _both & (1 << x) > 0 {
                [0x99, 0x66, 0x00, 0xff]
            } else if _zero & (1 << x) > 0 {
                [0xff, 0xcc, 0x00, 0xff]
            } else if _one & (1 << x) > 0 {
                [0xff, 0x66, 0x00, 0xff]
            } else {
                [0x66, 0x22, 0x00, 0xff]
            };
    
            pixel.copy_from_slice(&rgba);
        }
    }
}

// wasm_bindgen can't handle generics, so wrap the Frontend constructor to make it concrete
#[wasm_bindgen]
pub fn create_frontend(core: Chip8, keymap: Keymap, sync_mode: SyncModes) -> Frontend {
    Frontend::new(core, keymap, sync_mode)
}
