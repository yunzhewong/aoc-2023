use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_lines(path: &str) -> std::io::Lines<std::io::BufReader<std::fs::File>> {
    let file = File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);
    reader.lines()
}
