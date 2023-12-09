use std::fmt::Display;

pub struct Card {
    pub id: usize,
    pub winning_numbers: Vec<u64>,
    pub matches: Vec<u64>,
}

pub fn solve(input: &str) -> impl Display {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| get_card(id, line))
        .map(|c| {
            let matches = c.matches.len();
            if matches <= 1 {
                matches as u64
            } else {
                2_u64.pow((matches as u32) - 1).try_into().unwrap()
            }
        })
        .sum::<u64>()
}

pub fn get_card(id: usize, line: &str) -> Card {
    let mut split = line[8..].split(" | ");
    let winning_numbers = parse_numbers(&mut split);
    let owned_numbers = parse_numbers(&mut split);
    let matches = winning_numbers
        .iter()
        .cloned()
        .filter(|w| owned_numbers.contains(w))
        .collect::<Vec<u64>>()
        .clone();
    Card {
        id,
        winning_numbers,
        matches,
    }
}

fn parse_numbers<'a, I>(split: &mut I) -> Vec<u64>
where
    I: Iterator<Item = &'a str>,
{
    split
        .next()
        .unwrap()
        .split(" ")
        .into_iter()
        .map(|a| a.parse().unwrap_or(0))
        .filter(|n| *n != 0)
        .collect::<Vec<u64>>()
}
