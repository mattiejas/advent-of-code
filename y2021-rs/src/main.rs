use crate::day::Day;

mod day;
mod day06;
mod utils;

fn main() {
    println!("Hello, world!");

    println!("{}", day06::Day06::solve_part2());
}
