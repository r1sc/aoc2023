use std::{io::{BufReader, BufRead}, fs::File};

#[allow(dead_code)]
pub fn lines_from_test(data: &str) -> Vec<String> {
    data.lines().map(|l| l.to_string()).collect()
}

pub fn lines_from_file(path: &str) -> Vec<String> {
    BufReader::new(File::open(path).expect("Failed to find file"))
        .lines()
        .map(|l| l.unwrap())
        .collect()
}

pub fn string_from_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("Failed to find file")
}