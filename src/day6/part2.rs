use crate::day::part1::calculate_win_possibilities;
use std::fmt::Display;

pub fn solve(input: &str) -> impl Display {
    let mut lines = input.lines();
    let time = parse_line(lines.next().unwrap());
    let distance = parse_line(lines.next().unwrap());
    calculate_win_possibilities(time, distance + 1) as u64
}

fn parse_line(line: &str) -> i64 {
    line.chars()
        .skip(11)
        .collect::<String>()
        .replace(' ', "")
        .parse::<i64>()
        .unwrap()
}
