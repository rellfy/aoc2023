struct Point {
    x: usize,
    y: usize,
}

pub fn solve(input: &str) -> u64 {
    let symbols = get_symbol_points(input);
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| get_part_numbers(line, y, &symbols))
        .sum()
}

fn get_part_numbers(line: &str, y: usize, symbols: &[Point]) -> Vec<u64> {
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
        if !is_part_number(point, symbols) {
            continue;
        }
        parts.push(get_complete_number_at_index(line, x));
        should_skip_rest_of_part_number = true;
    }
    parts
}

fn is_part_number(part: Point, symbols: &[Point]) -> bool {
    for symbol in symbols {
        let y_diff = (symbol.y as isize - part.y as isize).abs();
        if y_diff > 1 {
            continue;
        }
        let x_diff = (symbol.x as isize - part.x as isize).abs();
        if x_diff > 1 {
            continue;
        }
        return true;
    }
    false
}

fn get_symbol_points(input: &str) -> Vec<Point> {
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

fn is_symbol(c: char) -> bool {
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
    format!("{backward_digits}{forward_digits}").parse().unwrap()
}
