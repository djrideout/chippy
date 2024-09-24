pub fn load_rom(path: &str) -> Vec<u8> {
    match std::fs::read(path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the ROM: {error:?}")
    }
}
