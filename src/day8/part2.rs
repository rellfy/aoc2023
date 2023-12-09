use std::fmt::Display;
use std::str::Chars;

pub fn solve(input: &str) -> impl Display {
    let mut lines = input.lines();
    let mut instructions = lines.next().unwrap().chars();
    let instructions_copy = instructions.clone();
    let maps = lines.skip(1).map(parse_line).collect::<Vec<_>>();
    let mut positions = maps
        .iter()
        .filter(|m| m[0].ends_with('A'))
        .map(|m| m[0])
        .collect::<Vec<_>>();
    let positions_length = positions.len();
    let mut steps = 0u128;
    let mut steps_to_z = Vec::new();
    'a: while positions.iter().any(|p| !p.ends_with('Z')) {
        let instruction = get_next_instruction(&mut instructions, &instructions_copy);
        steps += 1;
        for position in positions.iter_mut() {
            let map = maps.iter().find(|m| &m[0] == position).unwrap();
            match instruction {
                'L' => {
                    *position = map[1];
                }
                'R' => {
                    *position = map[2];
                }
                _ => panic!("invalid instruction"),
            }
            if position.ends_with('Z') {
                steps_to_z.push(steps);
                if steps_to_z.len() == positions_length {
                    break 'a;
                }
            }
        }
    }
    let first = steps_to_z.first().cloned().unwrap();
    let lcm = steps_to_z.into_iter().fold(first, |a, b| compute_lcm(a, b));
    lcm
}

fn parse_line(line: &str) -> [&str; 3] {
    let start = &line[0..3];
    let left = &line[7..10];
    let right = &line[12..15];
    [start, left, right]
}

fn get_next_instruction<'a>(instructions: &mut Chars<'a>, copy: &Chars<'a>) -> char {
    if let Some(instruction) = instructions.next() {
        instruction
    } else {
        *instructions = copy.clone();
        instructions.next().unwrap()
    }
}

fn compute_gcd(a: u128, b: u128) -> u128 {
    let next_b = a % b;
    let next_a = b;
    if next_b == 0 {
        next_a
    } else {
        compute_gcd(next_a, next_b)
    }
}

fn compute_lcm(a: u128, b: u128) -> u128 {
    (a * b) / compute_gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let value = compute_gcd(48, 18);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_lcm() {
        let value = compute_lcm(12, 15);
        assert_eq!(value, 60);
        let value2 = compute_lcm(value, 25);
        assert_eq!(value2, 300);
    }
}
