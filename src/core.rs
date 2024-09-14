use clap::ValueEnum;
use ::rand::prelude::*;

#[derive(ValueEnum, Debug, Clone, Default, PartialEq)]
pub enum Target {
    Chip, // This is "chip-8" in Gulrak's opcode table
    #[default]
    SuperModern, // This is "schipc" in Gulrak's opcode table
    SuperLegacy, // This is "schip-1.1" in Gulrak's opcode table
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
pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 64;
pub const PLANE_COUNT: usize = 2;

pub struct Chip8 {
    target: Target,
    clock: u32,
    // General purpose registers
    r_v: [u8; 16], // 16 general purpose "Vx" registers (x is 0-F)
    // Indexing registers
    r_i: usize, // Register "I"
    r_pc: usize, // Program counter
    r_sp: usize, // Stack pointer
    // Other registers
    r_delay: u8, // Delay timer
    r_sound: u8, // Sound timer
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
    pub high_res: bool, // For high-res resolution mode
    pub planes: [[u128; HEIGHT]; PLANE_COUNT],
    // Key press states
    pub prev_keys: [bool; 16],
    pub curr_keys: [bool; 16],
}

pub fn build_chip8(target: Target, clock: u32, rom: Vec<u8>) -> Chip8 {
    let mut chip8 = Chip8 {
        target,
        clock,
        r_v: [0; 16],
        r_i: 0,
        r_pc: 0x200,
        r_sp: 0,
        r_delay: 0,
        r_sound: 0,
        stack: [0; 16],
        mem: [0; 0x10000],
        halting: false,
        prev_op: 0,
        enabled_planes: 0b01,
        high_res: false,
        planes: [
            [0; HEIGHT],
            [0; HEIGHT]
        ],
        prev_keys: [false; 16],
        curr_keys: [false; 16]
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

    return chip8;
}

pub fn run_frame(chip8: &mut Chip8) {
    // Remaining instructions to run for this frame
    let mut remaining = chip8.clock;

    while remaining > 0 {
        remaining -= 1;

        // Get opcode
        let mut op = ((chip8.mem[chip8.r_pc] as u16) << 8) | chip8.mem[chip8.r_pc + 1] as u16;
        if chip8.halting {
            op = chip8.prev_op;
        } else {
            chip8.r_pc += 2;
        }

        // F000 is a 4-byte instruction, so if we need to skip an instruction and PC is on F000,
        // we should skip 4 bytes instead of 2.
        let _next_op = ((chip8.mem[chip8.r_pc] as u16) << 8) | chip8.mem[chip8.r_pc + 1] as u16;
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
                    for p in 0..PLANE_COUNT {
                        if (chip8.enabled_planes >> p) & 1 == 0 {
                            continue;
                        }
                        for i in 0 .. HEIGHT {
                            chip8.planes[p][i] = 0;
                        }
                    }
                }
                0x00EE => {
                    // 00EE - RET
                    // Return from a subroutine.
                    chip8.r_sp -= 1;
                    chip8.r_pc = chip8.stack[chip8.r_sp] as usize;
                }
                0x00FB if chip8.target != Target::Chip => {
                    // 00FB
                    // Scroll screen content right four pixels, in XO-CHIP only selected bit planes are scrolled
                    for p in 0..PLANE_COUNT {
                        if (chip8.enabled_planes >> p) & 1 == 0 {
                            continue;
                        }
                        for i in 0 .. HEIGHT {
                            chip8.planes[p][i] = chip8.planes[p][i] >> 4;
                        }
                    }
                }
                0x00FC if chip8.target != Target::Chip => {
                    // 00FC
                    // Scroll screen content left four pixels, in XO-CHIP only selected bit planes are scrolled
                    for p in 0..PLANE_COUNT {
                        if (chip8.enabled_planes >> p) & 1 == 0 {
                            continue;
                        }
                        for i in 0 .. HEIGHT {
                            chip8.planes[p][i] = chip8.planes[p][i] << 4;
                        }
                    }
                }
                0x00FD if chip8.target != Target::Chip => {
                    // 00FD - EXIT
                    // Exit the interpreter.
                    return;
                }
                0x00FE if chip8.target != Target::Chip => {
                    // 00FE - LOW
                    // Disable high-resolution mode.
                    chip8.high_res = false;
                    if chip8.target != Target::SuperLegacy {
                        for p in 0..PLANE_COUNT {
                            for i in 0 .. HEIGHT {
                                chip8.planes[p][i] = 0;
                            }
                        }
                    }
                }
                0x00FF if chip8.target != Target::Chip => {
                    // 00FF - HIGH
                    // Enable high-resolution mode.
                    chip8.high_res = true;
                    if chip8.target != Target::SuperLegacy {
                        for p in 0..PLANE_COUNT {
                            for i in 0 .. HEIGHT {
                                chip8.planes[p][i] = 0;
                            }
                        }
                    }
                }
                0xF000 if chip8.target == Target::XO => {
                    // F000
                    // Assign next 16 bit word to I, and set PC behind it. This is a four byte instruction.
                    chip8.r_i = ((chip8.mem[chip8.r_pc] as usize) << 8) | chip8.mem[chip8.r_pc + 1] as usize;
                    chip8.r_pc += 2;
                }
                _ => opcode_matched = false
            }
            if opcode_matched {
                break 'opcodes;
            }

            // 1-nibble param opcodes, but the param is the least significant nibble
            opcode_matched = true;
            match op & 0xFFF0 {
                0x00C0 if chip8.target != Target::Chip => {
                    // 00Cn
                    // Scroll screen content down N pixels, in XO-CHIP only selected bit planes are scrolled
                    let _count = _n >> (chip8.target == Target::SuperLegacy && !chip8.high_res) as u8;
                    for p in 0..PLANE_COUNT {
                        if (chip8.enabled_planes >> p) & 1 == 0 {
                            continue;
                        }
                        for i in (0..HEIGHT - 1).rev() {
                            if i < _count {
                                chip8.planes[p][i] = 0;
                                continue;
                            }
                            chip8.planes[p][i] = chip8.planes[p][i - _count];
                        }
                    }
                }
                0x00D0 if chip8.target == Target::XO => {
                    // 00Dn
                    // Scroll screen content up N hires pixel, in XO-CHIP only selected planes are scrolled
                    for p in 0..PLANE_COUNT {
                        if (chip8.enabled_planes >> p) & 1 == 0 {
                            continue;
                        }
                        for i in 0..HEIGHT - 1 {
                            if i + _n >= HEIGHT {
                                chip8.planes[p][i] = 0;
                                continue;
                            }
                            chip8.planes[p][i] = chip8.planes[p][i + _n];
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
                    if chip8.curr_keys[chip8.r_v[_x] as usize] {
                        chip8.r_pc += skip_count;
                    }
                }
                0xE0A1 => {
                    // ExA1 - SKNP Vx
                    // Skip next instruction if key with the value of Vx is not pressed.
                    if !chip8.curr_keys[chip8.r_v[_x] as usize] {
                        chip8.r_pc += skip_count;
                    }
                }
                0xF001 if chip8.target == Target::XO => {
                    // Fx01
                    // Select bit planes to draw on to x (not vX) when drawing with Dxy0/Dxyn
                    chip8.enabled_planes = (_x & 0b11) as u8;
                }
                0xF007 => {
                    // Fx07 - LD Vx, DT
                    // Set Vx = delay timer value.
                    chip8.r_v[_x] = chip8.r_delay;
                }
                0xF00A => {
                    // Fx0A - LD Vx, K
                    // Wait for a key press, store the value of the key in Vx.
                    chip8.halting = true;
                    // I guess I'll just grab the first key that releases between previous and current
                    for i in 0 ..= 0xF as usize {
                        if chip8.prev_keys[i] && !chip8.curr_keys[i] {
                            chip8.halting = false;
                            chip8.r_v[_x] = i as u8;
                            break;
                        }
                    }
                }
                0xF015 => {
                    // Fx15 - LD DT, Vx
                    // Set delay timer = Vx.
                    chip8.r_delay = chip8.r_v[_x];
                }
                0xF018 => {
                    // Fx18 - LD ST, Vx
                    // Set sound timer = Vx.
                    chip8.r_sound = chip8.r_v[_x];
                }
                0xF01E => {
                    // Fx1E - ADD I, Vx
                    // Set I = I + Vx.
                    chip8.r_i += chip8.r_v[_x] as usize;
                }
                0xF029 => {
                    // Fx29 - LD F, Vx
                    // Set I = location of 5-line sprite for digit Vx.
                    chip8.r_i = (chip8.r_v[_x] & 0xF) as usize * 5;
                }
                0xF030 if chip8.target != Target::Chip => {
                    // Fx30
                    // Set I = location of 10-line sprite for digit Vx.
                    chip8.r_i = SMALL_FONT_SET.len() + (chip8.r_v[_x] & 0xF) as usize * 10;
                }
                0xF033 => {
                    // Fx33 - LD B, Vx
                    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                    chip8.mem[chip8.r_i] = chip8.r_v[_x] / 100 % 10;
                    chip8.mem[chip8.r_i + 1] = chip8.r_v[_x] / 10 % 10;
                    chip8.mem[chip8.r_i + 2] = chip8.r_v[_x] % 10;
                }
                0xF055 => {
                    // Fx55 - LD [I], Vx
                    // Store registers V0 through Vx in memory starting at location I.
                    for i in 0 ..= _x {
                        chip8.mem[chip8.r_i + i] = chip8.r_v[i];
                    }
                    if chip8.target == Target::Chip || chip8.target == Target::XO {
                        chip8.r_i += _x + 1;
                    }
                }
                0xF065 => {
                    // Fx65 - LD Vx, [I]
                    // Read registers V0 through Vx from memory starting at location I.
                    for i in 0 ..= _x {
                        chip8.r_v[i] = chip8.mem[chip8.r_i + i];
                    }
                    if chip8.target == Target::Chip || chip8.target == Target::XO {
                        chip8.r_i += _x + 1;
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
                    if chip8.r_v[_x] == chip8.r_v[_y] {
                        chip8.r_pc += skip_count;
                    }
                }
                0x5002 if chip8.target == Target::XO => {
                    // 5xy2
                    // Write registers vX to vY to memory pointed to by I.
                    for _i in _x ..= _y {
                        chip8.mem[chip8.r_i + _i - _x] = chip8.r_v[_i];
                    }
                }
                0x5003 if chip8.target == Target::XO => {
                    // 5xy3
                    // Load registers vX to vY from memory pointed to by I.
                    for _i in _x ..= _y {
                        chip8.r_v[_i] = chip8.mem[chip8.r_i + _i - _x];
                    }
                }
                0x8000 => {
                    // 8xy0 - LD Vx, Vy
                    // Set Vx = Vy.
                    chip8.r_v[_x] = chip8.r_v[_y];
                }
                0x8001 => {
                    // 8xy1 - OR Vx, Vy
                    // Set Vx = Vx OR Vy.
                    chip8.r_v[_x] |= chip8.r_v[_y];
                    if chip8.target == Target::Chip {
                        chip8.r_v[0xF] = 0;
                    }
                }
                0x8002 => {
                    // 8xy2 - AND Vx, Vy
                    // Set Vx = Vx AND Vy.
                    chip8.r_v[_x] &= chip8.r_v[_y];
                    if chip8.target == Target::Chip {
                        chip8.r_v[0xF] = 0;
                    }
                }
                0x8003 => {
                    // 8xy3 - XOR Vx, Vy
                    // Set Vx = Vx XOR Vy.
                    chip8.r_v[_x] ^= chip8.r_v[_y];
                    if chip8.target == Target::Chip {
                        chip8.r_v[0xF] = 0;
                    }
                }
                0x8004 => {
                    // 8xy4 - ADD Vx, Vy
                    // Set Vx = Vx + Vy, set VF = carry.
                    let _next_x = chip8.r_v[_x] as u16 + chip8.r_v[_y] as u16;
                    chip8.r_v[_x] = _next_x as u8;
                    chip8.r_v[0xF] = (_next_x > 0xFF) as u8;
                }
                0x8005 => {
                    // 8xy5 - SUB Vx, Vy
                    // Set Vx = Vx - Vy, set VF = NOT borrow.
                    let _prev_x = chip8.r_v[_x];
                    if _prev_x < chip8.r_v[_y] {
                        chip8.r_v[_x] = !(chip8.r_v[_y] - _prev_x - 1);
                    } else {
                        chip8.r_v[_x] = _prev_x - chip8.r_v[_y];
                    }
                    chip8.r_v[0xF] = (_prev_x >= chip8.r_v[_y]) as u8;
                }
                0x8006 => {
                    // 8xy6 - SHR Vx {, Vy}
                    // Set Vx = Vx SHR 1.
                    let mut prev = chip8.r_v[_y];
                    if chip8.target == Target::SuperLegacy || chip8.target == Target::SuperModern {
                        prev = chip8.r_v[_x];
                    }
                    chip8.r_v[_x] = prev >> 1;
                    chip8.r_v[0xF] = prev & 1;
                }
                0x8007 => {
                    // 8xy7 - SUBN Vx, Vy
                    // Set Vx = Vy - Vx, set VF = NOT borrow.
                    let _prev_x = chip8.r_v[_x];
                    if chip8.r_v[_y] < _prev_x {
                        chip8.r_v[_x] = !(_prev_x - chip8.r_v[_y] - 1);
                    } else {
                        chip8.r_v[_x] = chip8.r_v[_y] - _prev_x;
                    }
                    chip8.r_v[0xF] = (chip8.r_v[_y] >= _prev_x) as u8;
                }
                0x800E => {
                    // 8xyE - SHL Vx {, Vy}
                    // Set Vx = Vx SHL 1.
                    let mut prev = chip8.r_v[_y];
                    if chip8.target == Target::SuperLegacy || chip8.target == Target::SuperModern {
                        prev = chip8.r_v[_x];
                    }
                    chip8.r_v[_x] = prev << 1;
                    chip8.r_v[0xF] = (prev & 0x80) >> 7;
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
                    chip8.r_pc = _nnn;
                }
                0x2000 => {
                    // 2nnn - CALL addr
                    // Call subroutine at nnn.
                    chip8.stack[chip8.r_sp] = chip8.r_pc as u16;
                    chip8.r_sp += 1;
                    chip8.r_pc = _nnn;
                }
                0x3000 => {
                    // 3xkk - SE Vx, byte
                    // Skip next instruction if Vx = kk.
                    if chip8.r_v[_x] == _kk {
                        chip8.r_pc += skip_count;
                    }
                }
                0x4000 => {
                    // 4xkk - SNE Vx, byte
                    // Skip next instruction if Vx != kk.
                    if chip8.r_v[_x] != _kk {
                        chip8.r_pc += skip_count;
                    }
                }
                0x6000 => {
                    // 6xkk - LD Vx, byte
                    // Set Vx = kk.
                    chip8.r_v[_x] = _kk;
                }
                0x7000 => {
                    // 7xkk - ADD Vx, byte
                    // Set Vx = Vx + kk.
                    let _next = chip8.r_v[_x] as u16 + _kk as u16;
                    chip8.r_v[_x] = _next as u8;
                }
                0x9000 => {
                    // 9xy0 - SNE Vx, Vy
                    // Skip next instruction if Vx != Vy.
                    if chip8.r_v[_x] != chip8.r_v[_y] {
                        chip8.r_pc += skip_count;
                    }
                }
                0xA000 => {
                    // Annn - LD I, addr
                    // Set I = nnn.
                    chip8.r_i = _nnn;
                }
                0xB000 => {
                    // Bnnn - JP V0, addr / Bxnn - JP Vx, addr
                    // Jump to location nnn + V0, or xnn + Vx on super.
                    let mut loc = chip8.r_v[0] as usize;
                    if chip8.target == Target::SuperLegacy || chip8.target == Target::SuperModern {
                        let _i = (_nnn & 0xF00) >> 8;
                        loc = chip8.r_v[_i] as usize;
                    }
                    chip8.r_pc = _nnn + loc;
                }
                0xC000 => {
                    // Cxkk - RND Vx, byte
                    // Set Vx = random byte AND kk.
                    chip8.r_v[_x] = thread_rng().gen::<u8>() & _kk;
                }
                0xD000 => {
                    // Dxyn - DRW Vx, Vy, nibble / Dxy0 - DRW Vx, Vy, 0
                    // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                    // Sprites are 8 pixels (8 bits/1 byte) wide and from 1 to 15 pixels in height,
                    // So each byte is one row of the sprite.
                    // Draws a 16x16 sprite if n=0 and platform is not CHIP-8. (8x16 on super legacy in low-res mode)
                    chip8.halting = true;
                    if remaining == 0
                        || chip8.target == Target::SuperModern
                        || chip8.target == Target::SuperLegacy && chip8.high_res
                        || chip8.target == Target::XO {
                        chip8.halting = false;
                        for p in 0..PLANE_COUNT {
                            if (chip8.enabled_planes >> p) & 1 == 0 {
                                continue;
                            }
                            let _x_mod = WIDTH >> !chip8.high_res as u8;
                            let _y_mod = HEIGHT >> !chip8.high_res as u8;
                            let _x_coord = chip8.r_v[_x] as usize % _x_mod;
                            let _y_coord = chip8.r_v[_y] as usize % _y_mod;
                            let mut sprite_height = (op & 0xF) as usize;
                            let mut sprite_width = 8 as usize;
                            if chip8.target != Target::Chip && sprite_height == 0 {
                                sprite_height = 16;
                                if chip8.target != Target::SuperLegacy || chip8.high_res {
                                    sprite_width = 16;
                                }
                            }
                            let mut unset = false;
                            for i in 0 .. sprite_height {
                                let mut row_i = _y_coord + i;
                                if row_i >= _y_mod {
                                    if chip8.target != Target::XO {
                                        continue;
                                    }
                                    row_i %= _y_mod;
                                }
                                let mut sprite_row = chip8.mem[(sprite_width >> 3) * i + chip8.r_i] as u128;
                                if sprite_width == 16 {
                                    sprite_row = (sprite_row << 8) | chip8.mem[(sprite_width >> 3) * i + chip8.r_i + 1] as u128;
                                }
                                let _curr = chip8.planes[p][row_i];
                                let _shift = WIDTH - 1 - _x_coord;
                                if _shift < sprite_width - 1 {
                                    chip8.planes[p][row_i] ^= sprite_row >> (sprite_width - 1 - _shift);
                                } else {
                                    chip8.planes[p][row_i] ^= sprite_row << (_shift - (sprite_width - 1));
                                }
                                if chip8.target == Target::XO && _x_coord > _x_mod - sprite_width {
                                    chip8.planes[p][row_i] ^= sprite_row.rotate_right((_x_coord - (_x_mod - sprite_width)) as u32) & (!0u16 as u128) << 112;
                                }
                                if !chip8.high_res {
                                    chip8.planes[p][row_i] &= (!0u64 as u128) << 64;
                                }
                                unset = unset || (!chip8.planes[p][row_i] & _curr) > 0;
                            }
                            chip8.r_v[0xF] = unset as u8;
                        }
                    }
                }
                _ => panic!("Unimplemented opcode 0x{:0x}", op)
            }
        }

        chip8.prev_op = op;
    }

    // Decrement timers.
    // They decrement at 60hz, so because fps is around 60, just decrement once per frame.
    if chip8.r_delay > 0 {
        chip8.r_delay -= 1;
    }
    if chip8.r_sound > 0 {
        chip8.r_sound -= 1;
    }
}
