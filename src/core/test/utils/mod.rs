use crate::core::{HEIGHT, WIDTH};

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
