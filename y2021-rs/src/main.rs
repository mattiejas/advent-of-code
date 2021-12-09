use crate::day::Day;

mod day;
mod day09;
mod utils;

fn main() {
    let now = std::time::Instant::now();

    println!("{}", day09::Day09::solve_part1());

    println!(
        "ran for {}ms",
        (now.elapsed().as_nanos()) as f32 / 1000000f32
    );
}
