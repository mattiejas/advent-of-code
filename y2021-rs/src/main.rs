use crate::day::Day;

mod day;
mod day10;
mod utils;

fn main() {
    let now = std::time::Instant::now();

    println!("{}", day10::Day10::solve_part1());

    println!(
        "ran for {}ms",
        (now.elapsed().as_nanos()) as f32 / 1000000f32
    );
}
