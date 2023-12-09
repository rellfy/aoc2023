use std::collections::HashMap;
use std::fmt::Display;
use std::iter::{IntoIterator, Iterator};

type Card = [u64; 5];

pub fn solve(input: &str) -> impl Display {
    let cards_and_bids = input.lines().map(parse_card_and_bid).collect::<Vec<_>>();
    let ranks = calculate_ranks(
        cards_and_bids
            .iter()
            .map(|(c, _)| c)
            .collect::<Vec<_>>()
            .as_slice(),
    );
    ranks
        .into_iter()
        .zip(cards_and_bids.into_iter().map(|(_, bid)| bid))
        .map(|(rank, bid)| rank * bid)
        .sum::<u64>()
}

fn parse_card_and_bid(line: &str) -> (Card, u64) {
    let mut values = line.split(' ').filter(|v| !v.is_empty());
    let card = parse_card(values.next().unwrap());
    let bid = values.next().unwrap().parse::<u64>().unwrap();
    (card, bid)
}

fn calculate_ranks(cards: &[&Card]) -> Vec<u64> {
    todo!()
}

fn parse_card(line: &str) -> Card {
    let mut cards = [0; 5];
    let mut input = line.chars().enumerate();
    while let Some((i, card)) = input.next() {
        cards[i] = get_card_number(card);
    }
    cards
}

fn get_card_number(c: char) -> u64 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("unknown card {c}"),
    }
}
