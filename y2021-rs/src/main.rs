use crate::day::Day;

mod day;
mod day08;
mod utils;

fn main() {
    let now = std::time::Instant::now();

    println!("{}", day08::Day08::solve_part2());

    println!(
        "ran for {}ms",
        (now.elapsed().as_nanos()) as f32 / 1000000f32
    );
}
