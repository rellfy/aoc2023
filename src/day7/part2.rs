use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::{IntoIterator, Iterator};

type Hand = [u64; 5];

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfKind = 3,
    FullHouse = 4,
    FourKind = 5,
    FiveKind = 6,
}

pub fn solve(input: &str) -> impl Display {
    let cards_and_bids = input.lines().map(parse_hand_and_bid).collect::<Vec<_>>();
    let sorted_hands = order_by_rank(&cards_and_bids);
    sorted_hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid, _))| (i as u64 + 1) * bid)
        .sum::<u64>()
}

fn parse_hand_and_bid(line: &str) -> (Hand, u64) {
    let mut values = line.split(' ').filter(|v| !v.is_empty());
    let hand = parse_card(values.next().unwrap());
    let bid = values.next().unwrap().parse::<u64>().unwrap();
    (hand, bid)
}

fn order_by_rank(hands_and_bids: &[(Hand, u64)]) -> Vec<(Hand, u64, HandType)> {
    let types = hands_and_bids.iter().map(|(h, _)| get_hand_type(h));
    let mut hands_bids_types = hands_and_bids
        .into_iter()
        .zip(types)
        .map(|((hand, bid), hand_type)| (hand, bid, hand_type))
        .collect::<Vec<(_, _, _)>>();
    hands_bids_types.sort_by(|(a_hand, _, a_type), (b_hand, _, b_type)| {
        order_hands(a_hand, a_type, b_hand, b_type)
    });
    hands_bids_types
        .into_iter()
        .map(|(hand, bid, hand_type)| (*hand, *bid, hand_type))
        .collect()
}

fn order_hands(a_hand: &Hand, a_type: &HandType, b_hand: &Hand, b_type: &HandType) -> Ordering {
    if a_type < b_type {
        return Ordering::Less;
    }
    if a_type > b_type {
        return Ordering::Greater;
    }
    let mut hands = a_hand.into_iter().zip(b_hand.into_iter());
    while let Some((a_card, b_card)) = hands.next() {
        if a_card < b_card {
            return Ordering::Less;
        }
        if a_card > b_card {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

fn get_hand_type(hand: &Hand) -> HandType {
    let mut card_frequencies = HashMap::<u64, u64>::new();
    for card in hand.iter() {
        let Some(card_frequency) = card_frequencies.get_mut(card) else {
            card_frequencies.insert(*card, 1);
            continue;
        };
        *card_frequency += 1;
    }
    let mut has_three_of_a_kind = false;
    let mut pair_count = 0u64;
    let mut j_frequency = card_frequencies
        .get(&get_card_number('J'))
        .cloned()
        .unwrap_or(0);
    let mut card_frequencies = card_frequencies
        .into_iter()
        .map(|(card, frequency)| (card, frequency))
        .collect::<Vec<(_, _)>>();
    card_frequencies.sort_by(|(a_card, a_freq), (b_card, b_freq)| {
        if a_freq > b_freq || *b_card == 0 {
            return Ordering::Less;
        }
        if b_freq > a_freq || *a_card == 0 {
            return Ordering::Greater;
        }
        Ordering::Equal
    });
    if j_frequency == 5 || j_frequency == 4 {
        return HandType::FiveKind;
    }
    for (card, frequency) in card_frequencies.into_iter() {
        if card == 0 {
            continue;
        }
        if frequency == 5 || frequency + j_frequency == 5 {
            return HandType::FiveKind;
        }
        if frequency == 4 || frequency + j_frequency == 4 {
            return HandType::FourKind;
        }
        if frequency == 3 || frequency + j_frequency == 3 {
            j_frequency -= 3 - frequency;
            has_three_of_a_kind = true;
            continue;
        }
        if frequency == 2 || frequency + j_frequency == 2 {
            j_frequency -= 2 - frequency;
            pair_count += 1;
        }
    }
    let is_full_house = has_three_of_a_kind && pair_count == 1;
    if is_full_house {
        return HandType::FullHouse;
    }
    if has_three_of_a_kind {
        return HandType::ThreeOfKind;
    }
    if pair_count == 2 {
        return HandType::TwoPair;
    }
    if pair_count == 1 {
        return HandType::OnePair;
    }
    HandType::HighCard
}

fn parse_card(line: &str) -> Hand {
    let mut hand = [0; 5];
    let mut input = line.chars().enumerate();
    while let Some((i, card)) = input.next() {
        hand[i] = get_card_number(card);
    }
    hand
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
        'J' => 0,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("unknown card {c}"),
    }
}
