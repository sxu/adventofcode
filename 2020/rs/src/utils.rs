use std::fs::File;
use std::io::{self, BufRead};

pub fn input_lines(input_path: &str) -> impl Iterator<Item = String> {
    let file =
        File::open(input_path).unwrap_or_else(|e| panic!("Failed to open {}: {}", input_path, e));
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap_or_else(|e| panic!("Failed to read line: {}", e)))
}
