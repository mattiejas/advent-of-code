use crate::day::Day;

mod day;
mod day07;
mod utils;

fn main() {
    let now = std::time::Instant::now();

    println!("{}", day07::Day07::solve_part2());

    println!(
        "ran for {}ms",
        (now.elapsed().as_nanos()) as f32 / 1000000f32
    );
}
