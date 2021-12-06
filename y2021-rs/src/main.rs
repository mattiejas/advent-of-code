use crate::day::Day;

mod day;
mod day01;
mod utils;

fn main() {
    let now = std::time::Instant::now();

    println!("{}", day01::Day01::solve_part2());

    println!(
        "ran for {}ms",
        (now.elapsed().as_nanos()) as f32 / 1000000f32
    );
}
