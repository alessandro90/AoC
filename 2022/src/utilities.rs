use std::fs;

pub fn read_file(fname: &str) -> String {
    fs::read_to_string("inputs/".to_owned() + fname + ".txt").expect("Invalid file path")
}
