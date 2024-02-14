use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();
    if let Ok(file) = File::open(filename) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                lines.push(line);
            }
        }
    }
    lines
}