pub mod part1;
mod part2;

static INPUT: &str = include_str!("input.txt");

pub fn run() -> (i64, i64) {
    (part1::solve(INPUT), part2::solve(INPUT))
}
