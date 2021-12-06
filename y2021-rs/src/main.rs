use crate::day::Day;

mod day;
mod day05;
mod utils;

fn main() {
    println!("Hello, world!");

    println!("{}", day05::Day05::solve_part2());
}
