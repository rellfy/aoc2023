mod part1;
mod part2;

static INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!("part 1 result: {}", part1::solve(INPUT));
    println!("part 2 result: {}", part2::solve(INPUT));
}
