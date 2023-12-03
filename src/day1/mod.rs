mod part1;

pub static INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!("part 1 result: {}", part1::solve(INPUT));
}
