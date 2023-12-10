use crate::IS_DEBUG;
use std::fmt::Display;

#[derive(Debug)]
struct Symbol {
    point: Point,
    c: char,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Connection {
    Top,
    Bottom,
    Right,
    Left,
    Any,
}

type Symbols = Vec<Vec<Symbol>>;

pub fn solve(input: &str) -> impl Display {
    let mut initial_point: Option<Point> = None;
    let symbols = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let point = Point {
                        x: x as isize,
                        y: y as isize,
                    };
                    if c == 'S' {
                        initial_point = Some(point);
                    }
                    Symbol { point, c }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let (mut symbol, mut from_connection) = get_next_symbol(
        0,
        Connection::Any,
        &initial_point.as_ref().unwrap(),
        &symbols,
    );
    let mut map = [[' '; 140]; 140];
    map[symbol.point.y as usize][symbol.point.x as usize] = symbol.c;
    let mut steps: u64 = 1;
    while symbol.c != 'S' {
        let (s, c) = get_next_symbol(steps, from_connection, &symbol.point, &symbols);
        symbol = s;
        from_connection = c;
        map[symbol.point.y as usize][symbol.point.x as usize] = symbol.c;
        steps += 1;
    }
    if IS_DEBUG {
        for y in map {
            println!("");
            for char in y {
                print!("{char}");
            }
        }
    }
    steps / 2
}

fn get_next_symbol<'a>(
    step: u64,
    from_connection: Connection,
    from_point: &Point,
    symbols: &'a Symbols,
) -> (&'a Symbol, Connection) {
    let from_symbol = symbols
        .get(from_point.y as usize)
        .unwrap()
        .get(from_point.x as usize)
        .unwrap();
    let free_connection = get_free_connection(from_connection, from_symbol);
    let surrounding_coordinates = get_surrounding_coordinates(&from_point);
    for coordinate in surrounding_coordinates.into_iter() {
        let Some(xs) = symbols.get(coordinate.y as usize) else {
            continue;
        };
        let Some(symbol) = xs.get(coordinate.x as usize) else {
            continue;
        };
        if symbol.c == 'S' && step < 2 {
            continue;
        }
        let Some(connections) = char_connections(symbol.c) else {
            continue;
        };
        for connection in connections.into_iter() {
            if !is_connection(free_connection, *from_point, connection, symbol.point) {
                continue;
            }
            if IS_DEBUG {
                println!(
                    "from free {} connection: {:#?} ({},{})",
                    from_symbol.c,
                    free_connection,
                    from_point.x + 1,
                    from_point.y + 1
                );
                println!("to {}: {:#?}", symbol.c, connection);
                println!("===============================");
            }
            return (symbol, connection);
        }
    }
    panic!("end of path")
}

fn is_connection(
    from_connection: Connection,
    from_point: Point,
    to_connection: Connection,
    to_point: Point,
) -> bool {
    match from_connection {
        Connection::Any => match to_connection {
            Connection::Top => is_b_at_top(to_point, from_point),
            Connection::Bottom => is_b_at_bottom(to_point, from_point),
            Connection::Right => is_b_at_right(to_point, from_point),
            Connection::Left => is_b_at_left(to_point, from_point),
            Connection::Any => true,
        },
        Connection::Top => match to_connection {
            Connection::Bottom => is_b_at_bottom(to_point, from_point),
            Connection::Any => is_b_at_top(from_point, to_point),
            _ => false,
        },
        Connection::Bottom => match to_connection {
            Connection::Top => is_b_at_top(to_point, from_point),
            Connection::Any => is_b_at_bottom(from_point, to_point),
            _ => false,
        },
        Connection::Left => match to_connection {
            Connection::Right => is_b_at_right(to_point, from_point),
            Connection::Any => is_b_at_left(from_point, to_point),
            _ => false,
        },
        Connection::Right => match to_connection {
            Connection::Left => is_b_at_left(to_point, from_point),
            Connection::Any => is_b_at_right(from_point, to_point),
            _ => false,
        },
    }
}

fn is_b_at_top(a: Point, b: Point) -> bool {
    a.x == b.x && b.y == a.y - 1
}

fn is_b_at_bottom(a: Point, b: Point) -> bool {
    a.x == b.x && b.y == a.y + 1
}

fn is_b_at_right(a: Point, b: Point) -> bool {
    a.y == b.y && b.x == a.x + 1
}

fn is_b_at_left(a: Point, b: Point) -> bool {
    a.y == b.y && b.x == a.x - 1
}

fn get_free_connection(from_connection: Connection, from_symbol: &Symbol) -> Connection {
    let connections = char_connections(from_symbol.c).unwrap();
    if connections[0] == from_connection {
        connections[1]
    } else {
        connections[0]
    }
}

fn char_connections(c: char) -> Option<[Connection; 2]> {
    Some(match c {
        'S' => [Connection::Any, Connection::Any],
        '|' => [Connection::Top, Connection::Bottom],
        '-' => [Connection::Left, Connection::Right],
        'L' => [Connection::Top, Connection::Right],
        'J' => [Connection::Top, Connection::Left],
        '7' => [Connection::Bottom, Connection::Left],
        'F' => [Connection::Bottom, Connection::Right],
        _ => {
            return None;
        }
    })
}

fn get_surrounding_coordinates(p: &Point) -> [Point; 4] {
    [
        Point { x: p.x + 1, y: p.y },
        Point { x: p.x - 1, y: p.y },
        Point { x: p.x, y: p.y + 1 },
        Point { x: p.x, y: p.y - 1 },
    ]
}
