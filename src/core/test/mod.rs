mod data;
mod utils;

use pretty_assertions::assert_eq;
use crate::core;

macro_rules! core_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (target, rom, expected, mem_val) = $value;
                let mut chip8 = core::Chip8::new(target, 16, rom.to_vec());
                chip8.mem[0x1FF] = mem_val; // Set correct mode without keypad input
                for _i in 0 .. 600 {
                    chip8.run_frame();
                }
                assert_eq!(utils::pretty_plane(&chip8.buffer_planes[0]), utils::pretty_plane(&expected));
                assert_eq!(utils::pretty_plane(&chip8.buffer_planes[1]), utils::pretty_plane(&data::EMPTY_PLANE));
            }
        )*
    }
}

core_tests! {
    test_1_chip8_logo_chip: (core::Target::Chip, data::LOGO, data::LOGO_PLANE, 0),
    test_1_chip8_logo_super_modern: (core::Target::SuperModern, data::LOGO, data::LOGO_PLANE, 0),
    test_1_chip8_logo_super_legacy: (core::Target::SuperLegacy, data::LOGO, data::LOGO_PLANE, 0),
    test_1_chip8_logo_xo: (core::Target::XO, data::LOGO, data::LOGO_PLANE, 0),

    test_2_ibm_logo_chip: (core::Target::Chip, data::IBM, data::IBM_PLANE, 0),
    test_2_ibm_logo_super_modern: (core::Target::SuperModern, data::IBM, data::IBM_PLANE, 0),
    test_2_ibm_logo_super_legacy: (core::Target::SuperLegacy, data::IBM, data::IBM_PLANE, 0),
    test_2_ibm_logo_xo: (core::Target::XO, data::IBM, data::IBM_PLANE, 0),

    test_3_corax_chip: (core::Target::Chip, data::CORAX, data::CORAX_PLANE, 0),
    test_3_corax_super_modern: (core::Target::SuperModern, data::CORAX, data::CORAX_PLANE, 0),
    test_3_corax_super_legacy: (core::Target::SuperLegacy, data::CORAX, data::CORAX_PLANE, 0),
    test_3_corax_xo: (core::Target::XO, data::CORAX, data::CORAX_PLANE, 0),

    test_4_flags_chip: (core::Target::Chip, data::FLAGS, data::FLAGS_PLANE, 0),
    test_4_flags_super_modern: (core::Target::SuperModern, data::FLAGS, data::FLAGS_PLANE, 0),
    test_4_flags_super_legacy: (core::Target::SuperLegacy, data::FLAGS, data::FLAGS_PLANE, 0),
    test_4_flags_xo: (core::Target::XO, data::FLAGS, data::FLAGS_PLANE, 0),

    test_5_quirks_chip: (core::Target::Chip, data::QUIRKS, data::QUIRKS_CHIP_PLANE, 1),
    test_5_quirks_super_modern: (core::Target::SuperModern, data::QUIRKS, data::QUIRKS_SUPER_MODERN_PLANE, 2),
    test_5_quirks_super_legacy: (core::Target::SuperLegacy, data::QUIRKS, data::QUIRKS_SUPER_LEGACY_PLANE, 4),
    test_5_quirks_xo: (core::Target::XO, data::QUIRKS, data::QUIRKS_XO_PLANE, 3),

    test_8_scrolling_super_modern_low: (core::Target::SuperModern, data::SCROLLING, data::SCROLLING_SUPER_MODERN_LOW_PLANE, 1),
    test_8_scrolling_super_modern_high: (core::Target::SuperModern, data::SCROLLING, data::SCROLLING_SUPER_HIGH_PLANE, 3),
    test_8_scrolling_super_legacy_low: (core::Target::SuperLegacy, data::SCROLLING, data::SCROLLING_SUPER_LEGACY_LOW_PLANE, 2),
    test_8_scrolling_super_legacy_high: (core::Target::SuperLegacy, data::SCROLLING, data::SCROLLING_SUPER_HIGH_PLANE, 3),
    test_8_scrolling_xo_low: (core::Target::XO, data::SCROLLING, data::SCROLLING_XO_LOW_PLANE, 4),
    test_8_scrolling_xo_high: (core::Target::XO, data::SCROLLING, data::SCROLLING_XO_HIGH_PLANE, 5),
}
