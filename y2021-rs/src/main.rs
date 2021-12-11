use crate::day::Day;

mod day;
mod day11;
mod utils;

fn main() {
    let now = std::time::Instant::now();

    println!("{}", day11::Day11::solve_part1());

    println!(
        "ran for {}ms",
        (now.elapsed().as_nanos()) as f32 / 1000000f32
    );
}
