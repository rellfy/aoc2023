use crate::day3::part1::{get_all_parts, get_symbol_points};

pub fn solve(input: &str) -> u64 {
    let symbols = get_symbol_points(input, is_gear_symbol);
    0
}

fn is_gear_symbol(c: char) -> bool {
    c == '*'
}
