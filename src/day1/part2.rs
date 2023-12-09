use std::fmt::Display;

static NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn solve(input: &str) -> impl Display {
    input.lines().map(|line| get_line_number(line)).sum::<u64>()
}

fn get_line_number(line: &str) -> u64 {
    let first_digit = get_first_digit(line);
    let last_digit = get_last_digit(line);
    format!("{first_digit}{last_digit}").parse().unwrap()
}

fn get_first_digit(line: &str) -> u64 {
    for i in 0..=line.len() {
        for j in 0..i {
            let substr = &line[j..i];
            if let Some(digit) = get_digit(substr) {
                return digit;
            }
        }
    }
    panic!("not found");
}

fn get_last_digit(line: &str) -> u64 {
    for i in (0..=line.len()).rev() {
        for j in (0..i).rev() {
            let substr = &line[j..i];
            if let Some(digit) = get_digit(substr) {
                return digit;
            }
        }
    }
    panic!("not found");
}

fn get_digit(substr: &str) -> Option<u64> {
    if substr.len() == 1 {
        if !substr.chars().next().unwrap().is_ascii_digit() {
            return None;
        }
        return substr.parse().ok();
    }
    let Some(number) = NUMBERS.iter().find(|n| n == &&substr) else {
        return None;
    };
    return Some(spelled_out_number_to_digit(number));
}

fn spelled_out_number_to_digit(number: &str) -> u64 {
    NUMBERS.iter().position(|n| n == &number).unwrap() as u64 + 1
}
