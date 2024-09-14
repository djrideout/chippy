use crate::core;

pub const EMPTY_PLANE: [u128; core::HEIGHT] = [0; core::HEIGHT];

pub const LOGO: &[u8] = include_bytes!("../../roms/1-chip8-logo.ch8");
pub const LOGO_FRAME_COUNT: u32 = 60;
pub const LOGO_PLANE: [u128; core::HEIGHT] = [
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000011111010000000000000000000010000000000110000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000100000110100011001110001110100100110010000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000100010101010100101001010010100101000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000100010100010111101001010010100100100000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000100010100010100001001010010100100010000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000100010100010011101001001110011101100000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000111110001100000001100111110000000000011111110000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000001111111011100000011101111111000000000111000111000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000011100011011100000011101110011100000001110000011000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000111000000011100000000001110001100000001110000011000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000111001010011100000001101110001100000001110000011000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000111000000011111100011101110001100000000111000110000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000111010001011111110011101110001101111000011111100000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000111001110011100111011101110011101111000111001110000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000111000000011100011011101111111000000001110000111000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000111000000011100011011101111110000000011100000011000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000111000000011100011011101110000000000011100000011000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000111000000011100011011101110101000111011100000011000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000011100011011100011011101110111000001011110000111000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000001111111011100011011101110001000110001111111110000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000111110011100011011101110001010111000111111100000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000001110011000110100000001100000010100001100000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000100100101000111000010001001000111010010000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000100111100100100000001001001010100011110000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000100100000010100000000101001010100010000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000100011101100011000011000111010011001110000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
];

pub const IBM: &[u8] = include_bytes!("../../roms/2-ibm-logo.ch8");
pub const IBM_FRAME_COUNT: u32 = 60;
pub const IBM_PLANE: [u128; core::HEIGHT] = [
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000011111111011111111100011111000000000111110010100000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000010100000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000011111111011111111111011111100000001111110001000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000111100000111000111000111110000011111000010100000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000011100000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000111100000111111100000111111101111111000000100000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000111100000111111100000111011111110111000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000111100000111000111000111001111100111000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000011100000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000011111111011111111111011111000111000111110000100000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000011000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000011111111011111111100011111000010000111110011100000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
    0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
];