use std::fmt::Display;

type IsSymbol = fn(char) -> bool;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

pub struct Part {
    pub number: u64,
    pub symbol: Point,
}

pub fn solve(input: &str) -> impl Display {
    let symbols = get_symbol_points(input, is_any_symbol);
    get_all_parts(input, &symbols)
        .into_iter()
        .map(|p| p.number)
        .sum::<u64>()
}

pub fn get_all_parts(input: &str, symbols: &[Point]) -> Vec<Part> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| get_parts(line, y, &symbols))
        .collect()
}

fn get_parts(line: &str, y: usize, symbols: &[Point]) -> Vec<Part> {
    let mut chars = line.chars().enumerate();
    let mut parts = Vec::new();
    let mut should_skip_rest_of_part_number = false;
    while let Some((x, c)) = chars.next() {
        if c.is_ascii_digit() && should_skip_rest_of_part_number {
            continue;
        }
        if !c.is_ascii_digit() {
            if should_skip_rest_of_part_number {
                should_skip_rest_of_part_number = false
            }
            continue;
        }
        let point = Point { x, y };
        let Some(symbol) = get_part_number_symbol(point, symbols) else {
            continue;
        };
        parts.push(Part {
            number: get_complete_number_at_index(line, x),
            symbol,
        });
        should_skip_rest_of_part_number = true;
    }
    parts
}

fn get_part_number_symbol(part: Point, symbols: &[Point]) -> Option<Point> {
    for symbol in symbols {
        let y_diff = (symbol.y as isize - part.y as isize).abs();
        if y_diff > 1 {
            continue;
        }
        let x_diff = (symbol.x as isize - part.x as isize).abs();
        if x_diff > 1 {
            continue;
        }
        return Some(symbol.clone());
    }
    None
}

pub fn get_symbol_points(input: &str, is_symbol: IsSymbol) -> Vec<Point> {
    let mut points = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !is_symbol(c) {
                continue;
            }
            points.push(Point { x, y });
        }
    }
    points
}

fn is_any_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn get_complete_number_at_index(line: &str, index: usize) -> u64 {
    // Get adjacent digits from start to index.
    let mut backward_chars = line.chars().rev().skip(line.len() - index);
    let mut backward_digits = String::new();
    while let Some(c) = backward_chars.next() {
        if !c.is_ascii_digit() {
            break;
        }
        backward_digits.push(c);
    }
    backward_digits = backward_digits.chars().rev().collect();
    // Get adjacent digits from index to end.
    let mut forward_chars = line.chars().skip(index);
    let mut forward_digits = String::new();
    while let Some(c) = forward_chars.next() {
        if !c.is_ascii_digit() {
            break;
        }
        forward_digits.push(c);
    }
    format!("{backward_digits}{forward_digits}")
        .parse()
        .unwrap()
}
