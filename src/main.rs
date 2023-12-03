#[cfg(feature = "day1")]
mod day1;

fn main() {
    #[cfg(feature = "day1")]
    day1::run();
}
