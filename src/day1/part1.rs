use std::fmt::Display;

pub fn solve(input: &str) -> impl Display {
    input.lines().map(|line| get_line_number(line)).sum::<u64>()
}

fn get_line_number(line: &str) -> u64 {
    let first_digit = get_first_digit(line);
    let last_digit = get_last_digit(line);
    let number_str = format!("{first_digit}{last_digit}");
    number_str.parse().unwrap()
}

fn get_first_digit(line: &str) -> char {
    get_digit(line.chars())
}

fn get_last_digit(line: &str) -> char {
    get_digit(line.chars().rev())
}

fn get_digit<I>(iter: I) -> char
where
    I: Iterator<Item = char>,
{
    for c in iter {
        if !c.is_ascii_digit() {
            continue;
        }
        return c;
    }
    panic!("no digit found");
}
