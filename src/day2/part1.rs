use std::str::Split;

const MAX_RED: u64 = 12;
const MAX_GREEN: u64 = 13;
const MAX_BLUE: u64 = 14;

pub struct Game {
    id: u64,
    pub red: u64,
    pub green: u64,
    pub blue: u64
}

pub fn solve(input: &str) -> u64 {
    parse_games(input)
        .into_iter()
        .filter(|g|
            g.red <= MAX_RED && g.green <= MAX_GREEN && g.blue <= MAX_BLUE
        )
        .map(|g| g.id)
        .sum()
}

pub fn parse_games(input: &str) -> Vec<Game> {
    let lines = input.lines();
    let mut games = Vec::new();
    for line in lines {
        let mut split = line.split(": ");
        let game_id = split.next().unwrap().replace("Game ", "").parse().unwrap();
        let sets = split.next().unwrap().split("; ");
        let (red, green, blue) = calculate_max_set_rgb(sets);
        games.push(Game {
            id: game_id,
            red,
            green,
            blue,
        })
    }
    games
}

fn calculate_max_set_rgb(sets: Split<&str>) -> (u64, u64, u64) {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for set in sets {
        let (r, g, b) = count_set_rgb(set);
        if r > max_red {
            max_red = r;
        }
        if g > max_green {
            max_green = g;
        }
        if b > max_blue {
            max_blue = b;
        }
    }
    (max_red, max_green, max_blue)
}

fn count_set_rgb(set: &str) -> (u64, u64, u64) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for item in set.split(", ") {
        let mut split = item.split(" ");
        let amount: u64 = split.next().unwrap().parse().unwrap();
        let colour = split.next().unwrap();
        match colour.chars().next().unwrap() {
            'r' => {
                red += amount;
            }
            'g' => {
                green += amount;
            }
            'b' => {
                blue += amount;
            }
            _ => {
                panic!("unknown colour: {colour}");
            }
        }
    }
    (red, green, blue)
}
