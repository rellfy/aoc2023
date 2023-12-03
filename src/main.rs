#[cfg(feature = "day1")]
mod day1;
#[cfg(feature = "day2")]
mod day2;

fn main() {
    #[cfg(feature = "day1")]
    day1::run();
    #[cfg(feature = "day2")]
    day2::run();
}
