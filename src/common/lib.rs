use std::fs::read_to_string;

pub fn read_input(path: &str) -> String {
  read_to_string(path).unwrap_or_else(|_| {
    return String::from("");
  })
}
