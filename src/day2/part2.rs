use crate::day2::part1::parse_games;

pub fn solve(input: &str) -> u64 {
    parse_games(input)
        .into_iter()
        .map(|g| g.red * g.green * g.blue)
        .sum()
}
