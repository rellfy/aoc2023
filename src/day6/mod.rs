pub mod part1;
mod part2;

static INPUT: &str = include_str!("input.txt");

pub fn run() -> (u64, u64) {
    (part1::solve(INPUT), part2::solve(INPUT))
}
