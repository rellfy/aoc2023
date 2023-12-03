pub fn solve(input: &str) -> u64 {
    0
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
