use crate::day::part1::calculate_win_possibilities;

pub fn solve(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = lines
        .next().unwrap().chars().skip(11).collect::<String>()
        .replace(' ', "")
        .parse::<i64>()
        .unwrap();
    let distance = lines
        .next().unwrap().chars().skip(11).collect::<String>()
        .replace(' ', "")
        .parse::<i64>()
        .unwrap();
    calculate_win_possibilities(time, distance) as u64
}
