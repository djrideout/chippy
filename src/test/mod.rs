mod data;

use pretty_assertions::assert_eq;
use crate::core;
use crate::utils::pretty_plane;

#[test]
fn test_1_chip8_logo_chip() {
    let mut chip8 = core::build_chip8(core::Target::Chip, 16, data::LOGO.to_vec());
    for _i in 0..data::LOGO_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::LOGO_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_1_chip8_logo_super_modern() {
    let mut chip8 = core::build_chip8(core::Target::SuperModern, 16, data::LOGO.to_vec());
    for _i in 0..data::LOGO_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::LOGO_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_1_chip8_logo_super_legacy() {
    let mut chip8 = core::build_chip8(core::Target::SuperLegacy, 16, data::LOGO.to_vec());
    for _i in 0..data::LOGO_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::LOGO_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_1_chip8_logo_xo() {
    let mut chip8 = core::build_chip8(core::Target::XO, 16, data::LOGO.to_vec());
    for _i in 0..data::LOGO_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::LOGO_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_2_ibm_logo_chip() {
    let mut chip8 = core::build_chip8(core::Target::Chip, 16, data::IBM.to_vec());
    for _i in 0..data::IBM_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::IBM_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_2_ibm_logo_super_modern() {
    let mut chip8 = core::build_chip8(core::Target::SuperModern, 16, data::IBM.to_vec());
    for _i in 0..data::IBM_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::IBM_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_2_ibm_logo_super_legacy() {
    let mut chip8 = core::build_chip8(core::Target::SuperLegacy, 16, data::IBM.to_vec());
    for _i in 0..data::IBM_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::IBM_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_2_ibm_logo_xo() {
    let mut chip8 = core::build_chip8(core::Target::XO, 16, data::IBM.to_vec());
    for _i in 0..data::IBM_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::IBM_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_3_corax_chip() {
    let mut chip8 = core::build_chip8(core::Target::Chip, 16, data::CORAX.to_vec());
    for _i in 0..data::CORAX_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::CORAX_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_3_corax_super_modern() {
    let mut chip8 = core::build_chip8(core::Target::SuperModern, 16, data::CORAX.to_vec());
    for _i in 0..data::CORAX_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::CORAX_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_3_corax_super_legacy() {
    let mut chip8 = core::build_chip8(core::Target::SuperLegacy, 16, data::CORAX.to_vec());
    for _i in 0..data::CORAX_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::CORAX_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_3_corax_xo() {
    let mut chip8 = core::build_chip8(core::Target::XO, 16, data::CORAX.to_vec());
    for _i in 0..data::CORAX_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::CORAX_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_4_flags_chip() {
    let mut chip8 = core::build_chip8(core::Target::Chip, 16, data::FLAGS.to_vec());
    for _i in 0..data::FLAGS_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::FLAGS_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_4_flags_super_modern() {
    let mut chip8 = core::build_chip8(core::Target::SuperModern, 16, data::FLAGS.to_vec());
    for _i in 0..data::FLAGS_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::FLAGS_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_4_flags_super_legacy() {
    let mut chip8 = core::build_chip8(core::Target::SuperLegacy, 16, data::FLAGS.to_vec());
    for _i in 0..data::FLAGS_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::FLAGS_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}

#[test]
fn test_4_flags_xo() {
    let mut chip8 = core::build_chip8(core::Target::XO, 16, data::FLAGS.to_vec());
    for _i in 0..data::FLAGS_FRAME_COUNT {
        core::run_frame(&mut chip8);
    }
    assert_eq!(pretty_plane(&chip8.planes[0]), pretty_plane(&data::FLAGS_PLANE));
    assert_eq!(pretty_plane(&chip8.planes[1]), pretty_plane(&data::EMPTY_PLANE));
}
