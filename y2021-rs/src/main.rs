use crate::day::Day;

mod day;
mod day06;
mod utils;

fn main() {
    let now = std::time::Instant::now();

    println!("{}", day06::Day06::solve_part2());

    println!("ran for {}ms", now.elapsed().as_millis());
}
