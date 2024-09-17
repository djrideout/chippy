mod data;

use pretty_assertions::assert_eq;
use crate::core;
use crate::utils::pretty_plane;

#[test]
fn test_1_chip8_logo_chip() {
    let mut chip8 = core::Chip8::new(core::Target::Chip, 16, data::LOGO.to_vec());
    for _i in 0..data::LOGO_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::LOGO_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_1_chip8_logo_super_modern() {
    let mut chip8 = core::Chip8::new(core::Target::SuperModern, 16, data::LOGO.to_vec());
    for _i in 0..data::LOGO_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::LOGO_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_1_chip8_logo_super_legacy() {
    let mut chip8 = core::Chip8::new(core::Target::SuperLegacy, 16, data::LOGO.to_vec());
    for _i in 0..data::LOGO_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::LOGO_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_1_chip8_logo_xo() {
    let mut chip8 = core::Chip8::new(core::Target::XO, 16, data::LOGO.to_vec());
    for _i in 0..data::LOGO_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::LOGO_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_2_ibm_logo_chip() {
    let mut chip8 = core::Chip8::new(core::Target::Chip, 16, data::IBM.to_vec());
    for _i in 0..data::IBM_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::IBM_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_2_ibm_logo_super_modern() {
    let mut chip8 = core::Chip8::new(core::Target::SuperModern, 16, data::IBM.to_vec());
    for _i in 0..data::IBM_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::IBM_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_2_ibm_logo_super_legacy() {
    let mut chip8 = core::Chip8::new(core::Target::SuperLegacy, 16, data::IBM.to_vec());
    for _i in 0..data::IBM_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::IBM_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_2_ibm_logo_xo() {
    let mut chip8 = core::Chip8::new(core::Target::XO, 16, data::IBM.to_vec());
    for _i in 0..data::IBM_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::IBM_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_3_corax_chip() {
    let mut chip8 = core::Chip8::new(core::Target::Chip, 16, data::CORAX.to_vec());
    for _i in 0..data::CORAX_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::CORAX_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_3_corax_super_modern() {
    let mut chip8 = core::Chip8::new(core::Target::SuperModern, 16, data::CORAX.to_vec());
    for _i in 0..data::CORAX_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::CORAX_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_3_corax_super_legacy() {
    let mut chip8 = core::Chip8::new(core::Target::SuperLegacy, 16, data::CORAX.to_vec());
    for _i in 0..data::CORAX_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::CORAX_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_3_corax_xo() {
    let mut chip8 = core::Chip8::new(core::Target::XO, 16, data::CORAX.to_vec());
    for _i in 0..data::CORAX_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::CORAX_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_4_flags_chip() {
    let mut chip8 = core::Chip8::new(core::Target::Chip, 16, data::FLAGS.to_vec());
    for _i in 0..data::FLAGS_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::FLAGS_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_4_flags_super_modern() {
    let mut chip8 = core::Chip8::new(core::Target::SuperModern, 16, data::FLAGS.to_vec());
    for _i in 0..data::FLAGS_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::FLAGS_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_4_flags_super_legacy() {
    let mut chip8 = core::Chip8::new(core::Target::SuperLegacy, 16, data::FLAGS.to_vec());
    for _i in 0..data::FLAGS_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::FLAGS_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_4_flags_xo() {
    let mut chip8 = core::Chip8::new(core::Target::XO, 16, data::FLAGS.to_vec());
    for _i in 0..data::FLAGS_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::FLAGS_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_5_quirks_chip() {
    let mut chip8 = core::Chip8::new(core::Target::Chip, 16, data::QUIRKS.to_vec());
    chip8.mem[0x1FF] = 1; // Set correct mode without keypad input
    for _i in 0..data::QUIRKS_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::QUIRKS_CHIP_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_5_quirks_super_modern() {
    let mut chip8 = core::Chip8::new(core::Target::SuperModern, 16, data::QUIRKS.to_vec());
    chip8.mem[0x1FF] = 2; // Set correct mode without keypad input
    for _i in 0..data::QUIRKS_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::QUIRKS_SUPER_MODERN_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_5_quirks_super_legacy() {
    let mut chip8 = core::Chip8::new(core::Target::SuperLegacy, 16, data::QUIRKS.to_vec());
    chip8.mem[0x1FF] = 4; // Set correct mode without keypad input
    for _i in 0..data::QUIRKS_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::QUIRKS_SUPER_LEGACY_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_5_quirks_xo() {
    let mut chip8 = core::Chip8::new(core::Target::XO, 16, data::QUIRKS.to_vec());
    chip8.mem[0x1FF] = 3; // Set correct mode without keypad input
    for _i in 0..data::QUIRKS_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::QUIRKS_XO_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_8_scrolling_super_modern_low() {
    let mut chip8 = core::Chip8::new(core::Target::SuperModern, 16, data::SCROLLING.to_vec());
    chip8.mem[0x1FF] = 1; // Set correct mode without keypad input
    for _i in 0..data::SCROLLING_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::SCROLLING_SUPER_MODERN_LOW_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_8_scrolling_super_modern_high() {
    let mut chip8 = core::Chip8::new(core::Target::SuperModern, 16, data::SCROLLING.to_vec());
    chip8.mem[0x1FF] = 3; // Set correct mode without keypad input
    for _i in 0..data::SCROLLING_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::SCROLLING_SUPER_HIGH_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_8_scrolling_super_legacy_low() {
    let mut chip8 = core::Chip8::new(core::Target::SuperLegacy, 16, data::SCROLLING.to_vec());
    chip8.mem[0x1FF] = 2; // Set correct mode without keypad input
    for _i in 0..data::SCROLLING_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::SCROLLING_SUPER_LEGACY_LOW_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_8_scrolling_super_legacy_high() {
    let mut chip8 = core::Chip8::new(core::Target::SuperLegacy, 16, data::SCROLLING.to_vec());
    chip8.mem[0x1FF] = 3; // Set correct mode without keypad input
    for _i in 0..data::SCROLLING_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::SCROLLING_SUPER_HIGH_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_8_scrolling_xo_low() {
    let mut chip8 = core::Chip8::new(core::Target::XO, 16, data::SCROLLING.to_vec());
    chip8.mem[0x1FF] = 4; // Set correct mode without keypad input
    for _i in 0..data::SCROLLING_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::SCROLLING_XO_LOW_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_8_scrolling_xo_high() {
    let mut chip8 = core::Chip8::new(core::Target::XO, 16, data::SCROLLING.to_vec());
    chip8.mem[0x1FF] = 5; // Set correct mode without keypad input
    for _i in 0..data::SCROLLING_FRAME_COUNT {
        chip8.run_frame();
    }
    assert_eq!(pretty_plane(&chip8.buffer_planes[0]), pretty_plane(&data::SCROLLING_XO_HIGH_PLANE));
    assert_eq!(pretty_plane(&chip8.buffer_planes[1]), pretty_plane(&data::EMPTY_PLANE));
}
