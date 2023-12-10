use crate::day::part1::{get_surrounding_coordinates, Point, Symbol, Symbols};
use crate::IS_DEBUG;
use std::collections::HashMap;
use std::fmt::Display;

const GRID_LIMIT_X: isize = 140;
const GRID_LIMIT_Y: isize = 140;

pub fn solve(input: &str) -> impl Display {
    let (mut symbols, _, _, points) = crate::day::part1::solve_outputs(input);
    clean_symbols_not_on_path(&mut symbols, &points);
    let mut scaled_symbols = scale_up(symbols.clone());
    // Fill path of scald symbols.
    let mut points = points.into_iter();
    let mut previous_point = points.next().unwrap();
    while let Some(point) = points.next() {
        let diff_x = point.x - previous_point.x;
        let new_x = match diff_x {
            1 => (previous_point.x * 2) + 1,
            -1 => (previous_point.x * 2) - 1,
            0 => point.x * 2,
            _ => panic!("invalid diff_x {diff_x}"),
        };
        let diff_y = point.y - previous_point.y;
        let new_y = match diff_y {
            1 => (previous_point.y * 2) + 1,
            -1 => (previous_point.y * 2) - 1,
            0 => point.y * 2,
            _ => panic!("invalid diff_y {diff_y}"),
        };
        let ys = scaled_symbols.get_mut(new_y as usize).unwrap();
        let s = ys.get_mut(new_x as usize).unwrap();
        s.c = '#';
        previous_point = point;
    }
    if IS_DEBUG {
        for y in scaled_symbols.iter() {
            println!("");
            for symbol in y {
                print!("{}", symbol.c);
            }
        }
        println!("");
    }
    let mut seen = HashMap::new();
    let _ = search_cluster(
        *scaled_symbols.get(0).unwrap().get(0).unwrap(),
        &scaled_symbols,
        &mut seen,
    )
    .unwrap_or(0);
    let all_count = symbols
        .iter()
        .flat_map(|ys| ys.iter().map(|s| if s.c == '.' { 1 } else { 0 }))
        .sum::<u64>();
    let all_scaled_symbols = scaled_symbols.iter().flat_map(|ys| ys).collect::<Vec<_>>();
    let mut outside_count = 0;
    for (_, outside) in seen.iter() {
        let Some(outside) = outside else {
            continue;
        };
        let mut found = false;
        for symbol in all_scaled_symbols.iter() {
            if symbol.point.x % 2 != 0 || symbol.point.y % 2 != 0 {
                // Not an original (unscaled) symbol.
                continue;
            }
            if symbol.point == *outside {
                found = true;
                break;
            }
        }
        if found {
            outside_count += 1;
        }
    }
    all_count - outside_count
}

fn clean_symbols_not_on_path(symbols: &mut Symbols, original_points: &[Point]) {
    for (_, ys) in symbols.iter_mut().enumerate() {
        for (_, symbol) in ys.iter_mut().enumerate() {
            let is_on_path = original_points.iter().any(|p| *p == symbol.point);
            if !is_on_path {
                symbol.c = '.';
            }
        }
    }
}

fn scale_up(original_symbols: Symbols) -> Symbols {
    let mut symbols = [['.'; GRID_LIMIT_X as usize * 2]; GRID_LIMIT_Y as usize * 2];
    for (y, ys) in original_symbols.iter().enumerate() {
        for (x, symbol) in ys.iter().enumerate() {
            let scaled_x = x * 2;
            let scaled_y = y * 2;
            symbols[scaled_y][scaled_x] = symbol.c;
        }
    }
    symbols
        .to_vec()
        .into_iter()
        .enumerate()
        .map(|(y, ys)| {
            ys.into_iter()
                .enumerate()
                .map(|(x, c)| Symbol {
                    point: Point {
                        x: x as isize,
                        y: y as isize,
                    },
                    c,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn search_cluster(
    initial_symbol: Symbol,
    symbols: &Symbols,
    seen: &mut HashMap<(isize, isize), Option<Point>>,
) -> Option<u64> {
    let mut queue = vec![initial_symbol];
    let mut i: usize = 0;
    let mut count: u64 = 0;
    while let Some(symbol) = queue.get(i).cloned() {
        let Some(result_count) = process_cluster_queue(&mut queue, &symbol, symbols, seen) else {
            return None;
        };
        count += result_count;
        i += 1;
    }
    Some(count)
}

fn process_cluster_queue(
    queue: &mut Vec<Symbol>,
    symbol: &Symbol,
    symbols: &Symbols,
    seen: &mut HashMap<(isize, isize), Option<Point>>,
) -> Option<u64> {
    if seen.get(&(symbol.point.x, symbol.point.y)).is_some() {
        return Some(0);
    }
    seen.insert((symbol.point.x, symbol.point.y), None);
    let is_ground = symbol.c == '.';
    if !is_ground {
        return Some(0);
    }
    let count = 1;
    seen.insert((symbol.point.x, symbol.point.y), Some(symbol.point));
    for neighbour_point in get_surrounding_coordinates(&symbol.point) {
        let Some(ys) = symbols.get(neighbour_point.y as usize) else {
            continue;
        };
        let Some(neighbour) = ys.get(neighbour_point.x as usize) else {
            continue;
        };
        queue.push(*neighbour);
    }
    Some(count)
}
