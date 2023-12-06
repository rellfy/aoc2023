pub struct Maps {
    maps: Vec<Map>
}

#[derive(Default)]
struct Map {
    rows: Vec<MapRow>,
}

#[derive(Default)]
struct MapRow {
    source: u64,
    destination: u64,
    length: u64,
}

pub fn solve(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds = lines
        .next().unwrap().split(" ").skip(1)
        .map(|str| str.parse::<u64>().unwrap());
    let maps = Maps::parse(lines.skip(2));
    let mut min_location: i64 = i64::MAX;
    for seed in seeds {
        let output = maps.get_output(seed as i64);
        if output < min_location {
            min_location = output;
        }
    }
    min_location as u64
}

impl Maps {
    pub fn parse<'a, I>(mut lines: I) -> Self where I: Iterator<Item = &'a str> {
        let mut maps = Vec::new();
        let mut i = 0;
        let mut should_skip_line = false;
        while let Some(line) = lines.next() {
            if should_skip_line {
                should_skip_line = false;
                continue;
            }
            if line.chars().next().is_none() {
                // This is a blank line, the next one is the map title.
                should_skip_line = true;
                i += 1;
                continue;
            };
            let map = {
                if maps.len() < i + 1 {
                    maps.push(Map::default());
                }
                maps.get_mut(i).unwrap()
            };
            let mut row = line.split(' ').map(|value| value.parse::<u64>().unwrap());
            map.rows.push(MapRow {
                destination: row.next().unwrap(),
                source: row.next().unwrap(),
                length: row.next().unwrap(),
            });
        }
        Self {
            maps
        }
    }

    pub fn get_output(&self, input: i64) -> i64 {
        let mut output = input;
        for map in self.maps.iter() {
            output = map.map(output);
        }
        output
    }
}

impl Map {
    pub fn map(&self, input: i64) -> i64 {
        for map in self.rows.iter() {
            let min = map.source;
            let max = map.source + map.length;
            if input < min as i64 || input >= max as i64 {
                continue;
            }
            let mapping = map.destination as i64 - map.source as i64;
            return input + mapping;
        }
        input
    }
}
