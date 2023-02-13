use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn priority(item: char) -> u32 {
    let offset = if item.is_lowercase() { 96 } else { 38 };
    item as u32 - offset
}

fn parse_line(s: &str) -> Vec<u32> {
    s.chars().map(priority).collect()
}

fn parse_input(filename: &str) -> Vec<Vec<u32>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .collect()
}

fn intersect(a: HashSet<u32>, b: &[u32]) -> HashSet<u32> {
    let b: HashSet<u32> = HashSet::from_iter(b.iter().cloned());
    &a & &b
}

fn intersect2(a: &[u32], b: &[u32]) -> u32 {
    // really understand why 'x.iter().cloned()' is needed and not
    // just 'x'; however without the extra methods the return type is
    // HashSet<&u32> and we get lots of fun lifecycle issues.
    let a: HashSet<u32> = HashSet::from_iter(a.iter().cloned());
    unique_entry(intersect(a, b))
    // let a: HashSet<u32> = HashSet::from_iter(a.iter().cloned());
    // let b: HashSet<u32> = HashSet::from_iter(b.iter().cloned());
    // unique_entry(&a & &b)
}

fn intersect3(a: &[u32], b: &[u32], c: &[u32]) -> u32 { // u32
    let a: HashSet<u32> = HashSet::from_iter(a.iter().cloned());
    unique_entry(intersect(intersect(a, b), c))
    // let b: HashSet<u32> = HashSet::from_iter(b.iter().cloned());
    // let c: HashSet<u32> = HashSet::from_iter(c.iter().cloned());
    // unique_entry(&(&a & &b) & &c)
}

fn unique_entry(s: HashSet<u32>) -> u32 {
    match s.iter().next() {
        Some(&x) => x,
        None => panic!("no entry found"),
    }
}

fn part1fn(el: &Vec<u32>) -> u32 {
    let n = el.len() / 2;
    intersect2(&el[0..n], &el[n..])
}

fn part1(data: &[Vec<u32>]) -> u32 {
    data.iter().map(part1fn).sum()
}

fn part2(data: &[Vec<u32>]) -> u32 {
    let mut ret = 0;
    let mut lines = data.iter();
    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        ret += intersect3(a, b, c);
    }
    ret
}

fn main() {
    let dat = parse_input("data/input-d03.txt");
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
        let dat = parse_input("data/test-input-d03.txt");
        assert_eq!(part1(&dat), 157);
        assert_eq!(part2(&dat), 70);
    }
}
