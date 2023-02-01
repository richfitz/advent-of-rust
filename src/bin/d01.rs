use std::fs::File;
use std::io::{self, BufRead};

fn parse_input(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let mut ret: Vec<i32> = Vec::new();
    let mut n = 0;
    for line in io::BufReader::new(file).lines() {
        let el = line.unwrap();
        if el.is_empty() {
            ret.push(n);
            n = 0;
        } else {
            n += el.trim().parse::<i32>().expect("Invalid input");
        }
    }
    ret.push(n);
    ret
}

fn part1(data: &Vec<i32>) -> i32 {
    return *data.iter().max().unwrap();
}

fn part2(data: &Vec<i32>) -> i32 {
    let mut data = data.clone();
    data.sort_by(|a, b| b.cmp(a));
    data[0] + data[1] + data[2]
}

fn main() {
    let dat = parse_input("data/input-d01.txt");
    let ans1 = part1(&dat);
    let ans2 = part2(&dat);
    println!("Part 1: {ans1}");
    println!("Part 2: {ans2}");
}