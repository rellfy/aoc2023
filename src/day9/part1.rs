
pub fn solve(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.split(' ').map(|v| v.parse::<u64>().unwrap()).collect::<Vec<u64>>())
        .map(|values| find_next_value(&values))
        .sum()
}

fn find_next_value(values: &[u64]) -> u64 {
    0
}
