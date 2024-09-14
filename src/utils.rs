use macroquad::prelude::load_file;
use crate::core::{HEIGHT, WIDTH};

pub async fn load_rom(path: &str) -> Vec<u8> {
    let _result = load_file(path).await;
    let _rom = match _result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the ROM: {error:?}")
    };
    return _rom;
}

pub fn pretty_plane(plane: &[u128]) -> String {
    let mut output = String::new();
    for _i in 0..HEIGHT {
        output.push_str(format!("{:0width$b}", plane[_i], width = WIDTH)
            .replace("0", "▯")
            .replace("1", "▮")
            .as_str());
        output.push('\n');
    }
    return output;
}
