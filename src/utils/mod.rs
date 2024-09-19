use macroquad::prelude::load_file;

pub async fn load_rom(path: &str) -> Vec<u8> {
    let _result = load_file(path).await;
    let _rom = match _result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the ROM: {error:?}")
    };
    return _rom;
}
