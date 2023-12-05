#[cfg(feature = "day1")]
mod day1;
#[cfg(feature = "day2")]
mod day2;
#[cfg(feature = "day3")]
mod day3;
#[cfg(feature = "day4")]
mod day4;

fn main() {
    #[cfg(feature = "day1")]
    day1::run();
    #[cfg(feature = "day2")]
    day2::run();
    #[cfg(feature = "day3")]
    day3::run();
    #[cfg(feature = "day4")]
    day4::run();
}
