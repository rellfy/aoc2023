use std::time::{Instant};
#[cfg(feature = "day1")]
#[path = "day1/mod.rs"]
mod day;
#[cfg(feature = "day2")]
#[path = "day2/mod.rs"]
mod day;
#[cfg(feature = "day3")]
#[path = "day3/mod.rs"]
mod day;
#[cfg(feature = "day4")]
#[path = "day4/mod.rs"]
mod day;
#[cfg(feature = "day5")]
#[path = "day5/mod.rs"]
mod day;
#[cfg(feature = "day6")]
#[path = "day6/mod.rs"]
mod day;

fn main() {
    let (p1, p2) = day::run();
    println!("part 1 result: {p1}");
    println!("part 2 result: {p2}");
    #[cfg(feature = "benchmark")]
    run_benchmark();
}

#[cfg(feature = "benchmark")]
fn run_benchmark() {
    let micros = benchmark(day::run, 1000);
    println!("benchmark: {micros} microseconds");
}

fn benchmark<F>(func: F, iterations: u128) -> u128 where F: Fn() -> (u64, u64) {
    (0..iterations).map(|_| {
        let a = Instant::now();
        func();
        Instant::now().duration_since(a).as_micros()
    }).sum::<u128>() / iterations
}
