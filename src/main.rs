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

fn main() {
    let (p1, p2) = day::run();
    println!("part 1 result: {p1}");
    println!("part 2 result: {p2}");
}
