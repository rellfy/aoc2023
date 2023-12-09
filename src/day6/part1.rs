
pub fn solve(input: &str) -> u64 {
    let mut lines = input.lines();
    let times = parse_line(lines.next().unwrap());
    let distances = parse_line(lines.next().unwrap());
    times.zip(distances)
        .map(|(time, distance)| calculate_win_possibilities(time, distance))
        .product::<i64>() as u64
}

fn parse_line(line: &str) -> impl Iterator<Item = i64> + '_ {
    line.split(' ').filter_map(|n| n.parse::<i64>().ok())
}

pub fn calculate_win_possibilities(time: i64, distance: i64) -> i64 {
    let (max, min) = solve_for_tb(time, 0, distance);
    max.ceil() as i64 - min.floor() as i64 - 1
}

fn solve_for_tb(t: i64, vi: i64, d: i64) -> (f64, f64) {
    let a = -1;
    let b = t - vi;
    let c = vi*t - d;
    solve_bhaskara(a as f64, b as f64, c as f64)
}

fn solve_bhaskara(a: f64, b: f64, c: f64) -> (f64, f64) {
    let sqrt = (b.powf(2.0) - 4.0*a*c).sqrt();
    let max = (-b - sqrt) / (2.0*a);
    let min = (-b + sqrt) / (2.0*a);
    (max, min)
}
