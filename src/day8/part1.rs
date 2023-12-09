use std::fmt::Display;
use std::str::Chars;

pub fn solve(input: &str) -> impl Display {
    let mut lines = input.lines();
    let mut instructions = lines.next().unwrap().chars();
    let instructions_copy = instructions.clone();
    let maps = lines.skip(1).map(parse_line).collect::<Vec<_>>();
    let mut position = "AAA";
    let mut steps = 0;
    while position != "ZZZ" {
        let instruction = get_next_instruction(&mut instructions, &instructions_copy);
        let map = maps.iter().find(|m| m[0] == position).unwrap();
        match instruction {
            'L' => {
                position = map[1];
            }
            'R' => {
                position = map[2];
            }
            _ => panic!("invalid instruction"),
        }
        steps += 1;
    }
    steps
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
