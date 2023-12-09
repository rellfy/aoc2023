use std::fmt::Display;

use crate::day::part1::{get_all_parts, get_symbol_points, Part, Point};

pub fn solve(input: &str) -> impl Display {
    let gears = get_symbol_points(input, is_gear_symbol);
    let parts = get_all_parts(input, &gears);
    gears
        .iter()
        .map(|gear| calculate_gear_ratio(gear, &parts))
        .sum::<u64>()
}

fn calculate_gear_ratio(gear: &Point, parts: &[Part]) -> u64 {
    let gear_parts = parts
        .iter()
        .filter(|p| p.symbol == *gear)
        .collect::<Vec<&Part>>();
    if gear_parts.len() <= 1 {
        0
    } else {
        gear_parts.into_iter().map(|p| p.number).product()
    }
}

fn is_gear_symbol(c: char) -> bool {
    c == '*'
}
