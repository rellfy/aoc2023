use crate::day::part1::parse_games;
use std::fmt::Display;

pub fn solve(input: &str) -> impl Display {
    parse_games(input)
        .into_iter()
        .map(|g| g.red * g.green * g.blue)
        .sum::<u64>()
}
