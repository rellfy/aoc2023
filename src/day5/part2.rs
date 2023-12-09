use crate::day::part1::Maps;
use rayon::prelude::*;
use std::fmt::Display;

pub fn solve(input: &str) -> impl Display {
    let mut lines = input.lines();
    let mut seed_input = lines
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|str| str.parse::<u64>().unwrap());
    let mut seed_ranges = Vec::<(u64, u64)>::new();
    while let Some(start) = seed_input.next() {
        let count = seed_input.next().unwrap();
        seed_ranges.push((start, count));
    }
    let maps = Maps::parse(lines.skip(2));
    seed_ranges
        .into_par_iter()
        .map(|(start, count)| {
            (start..(start + count))
                .into_par_iter()
                .map(|seed| maps.get_output(seed as i64))
                .min()
                .unwrap()
        })
        .min()
        .unwrap() as u64
}
