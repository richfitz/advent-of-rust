use std::fs::File;
use std::io::{self, BufRead};

struct Round {
    them: i32,
    us: i32,
}

type Match = Vec<Round>;

fn parse_input(filename: &str) -> Match {
    let file = File::open(filename).unwrap();
    let mut ret: Match = Vec::new();
    for line in io::BufReader::new(file).lines() {
        ret.push(parse_line(&line.unwrap()));
    }
    ret
}

fn parse_line(s: &str) -> Round {
    Round {
        them: parse_el(s.chars().next().unwrap()),
        us: parse_el(s.chars().nth(2).unwrap())
    }
}

fn parse_el(c: char) -> i32 {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'X' => 0,
        'Y' => 1,
        'Z' => 2,
        _ => panic!("invalid entry")
    }
}

fn part1(data: &Match) -> i32 {
    data.iter()
        .map(|d| ((d.us - d.them + 1).rem_euclid(3)) * 3 + d.us + 1)
        .sum()
}

fn part2(data: &Match) -> i32 {
    data.iter()
        .map(|d| d.us * 3 + (d.them + d.us - 1).rem_euclid(3) + 1)
        .sum()
}

fn main() {
    let dat = parse_input("data/input-d02.txt");
    let ans1 = part1(&dat);
    let ans2 = part2(&dat);
    println!("Part 1: {ans1}");
    println!("Part 2: {ans2}");
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let dat = parse_input("data/test-input-d02.txt");
        assert_eq!(part1(&dat), 15);
        assert_eq!(part2(&dat), 12);
    }
}
