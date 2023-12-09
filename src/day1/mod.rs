use std::fmt::Display;
mod part1;
mod part2;

static INPUT: &str = include_str!("input.txt");

pub fn run() -> (impl Display, impl Display) {
    (part1::solve(INPUT), part2::solve(INPUT))
}
