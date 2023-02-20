use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_input<T>(filename: &str, transform: impl Fn(&str) -> T) -> Vec<T> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| transform(&line.unwrap()))
        .collect()
}
