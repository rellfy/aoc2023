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
    day::run();
}
