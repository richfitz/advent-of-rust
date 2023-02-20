struct Pair {
    a: (i32, i32),
    b: (i32, i32),
}

fn parse_line(s: &str) -> Pair {
    let mut digits = s.split(|c| c == ',' || c == '-')
        .map(|x| x.parse::<i32>().unwrap());
    let a = (digits.next().unwrap(), digits.next().unwrap());
    let b = (digits.next().unwrap(), digits.next().unwrap());
    Pair { a, b }
}

fn is_fully_contained(p: &Pair) -> bool {
    (p.a.0 <= p.b.0 && p.a.1 >= p.b.1) || (p.a.0 >= p.b.0 && p.a.1 <= p.b.1)
}

fn is_overlapping(p: &Pair) -> bool {
    p.a.0 <= p.b.1 && p.a.1 >= p.b.0
}

fn count_if(data: &[Pair], f: impl Fn(&Pair) -> bool) -> usize {
    data.iter()
        .map(f)
        .filter(|el| *el)
        .count()
}

fn part1(data: &[Pair]) -> usize {
    count_if(data, is_fully_contained)
}

fn part2(data: &[Pair]) -> usize {
    count_if(data, is_overlapping)
}

fn main() {
    let dat = aoc::parse_input("data/input-d04.txt", parse_line);
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
        let dat = parse_input("data/test-input-d04.txt");
        assert_eq!(part1(&dat), 2);
        assert_eq!(part2(&dat), 4);
    }
}
