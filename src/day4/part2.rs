use crate::day::part1::get_card;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Default)]
struct Copies {
    data: HashMap<usize, usize>,
}

pub fn solve(input: &str) -> impl Display {
    let mut copies = Copies::default();
    let mut total = 0;
    input.lines().enumerate().for_each(|(id, line)| {
        let card = get_card(id, line);
        let matches = card.matches.len();
        let card_copies = copies.read(id);
        let instances = card_copies + 1;
        copies.store_card(id, matches, instances);
        total += 1 + (instances * matches);
    });
    total as u64
}

impl Copies {
    pub fn store_card(&mut self, index: usize, matches: usize, instances: usize) {
        for i in index + 1..=index + matches {
            let Some(copies) = self.data.get_mut(&i) else {
                self.data.insert(i, instances);
                continue;
            };
            *copies += instances;
        }
    }

    pub fn read(&self, card_index: usize) -> usize {
        self.data.get(&card_index).cloned().unwrap_or(0)
    }
}
