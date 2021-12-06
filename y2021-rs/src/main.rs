use crate::day::Day;

mod day;
mod day04;
mod utils;

fn main() {
    let now = std::time::Instant::now();

    println!("{}", day04::Day04::solve_part2());

    println!(
        "ran for {}ms",
        (now.elapsed().as_nanos()) as f32 / 1000000f32
    );
}
