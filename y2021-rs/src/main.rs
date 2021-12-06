use crate::day::Day;

mod day;
mod day02;
mod utils;

fn main() {
    let now = std::time::Instant::now();

    println!("{}", day02::Day02::solve_part2());

    println!(
        "ran for {}ms",
        (now.elapsed().as_nanos()) as f32 / 1000000f32
    );
}
