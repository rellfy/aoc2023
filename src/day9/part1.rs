
pub fn solve(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.split(' ').map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .map(|values| find_next_value(values))
        .map(|a| a)
        .sum::<i64>()
}

fn find_next_value(values: Vec<i64>) -> i64 {
    let mut diffs = vec![values];
    while diffs.last().unwrap().iter().sum::<i64>() != 0 {
        let last_diff = diffs.last().unwrap();
        let new_diff = calculate_diff(last_diff);
        diffs.push(new_diff);
    }
    diffs.iter().for_each(|d| println!("{:?}", d));
    diffs.into_iter().map(|diff| diff.last().cloned().unwrap()).sum()
}

fn calculate_diff(values: &[i64]) -> Vec<i64> {
    let mut diff = Vec::with_capacity(values.len() - 1);
    for (i, value) in values.iter().skip(1).enumerate() {
        let previous = values.get(i).unwrap();
        diff.push(value - previous);
    }
    diff
}
