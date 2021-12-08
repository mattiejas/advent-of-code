use crate::day;
use crate::utils;

pub struct Day01 {}

fn get_measurements() -> Vec<i32> {
    let input = utils::input(1, false);
    return input
        .split('\n')
        .map(|x| x.parse::<i32>())
        .flatten()
        .collect::<Vec<i32>>();
}

fn count_increasing_measurements(measurements: &Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 1..measurements.len() {
        if measurements[i] > measurements[i - 1] {
            count += 1;
        }
    }
    return count;
}

fn get_sliding_windows(measurements: &Vec<i32>, w: usize) -> Vec<Vec<i32>> {
    let mut windows = Vec::new();
    for i in 0..measurements.len() {
        let mut window = Vec::new();
        if i + w <= measurements.len() {
            for j in i..(i + w) {
                window.push(measurements[j]);
            }
        }
        windows.push(window);
    }
    return windows;
}

impl day::Day for Day01 {
    fn solve_part1() -> String {
        let measurements = get_measurements();
        return count_increasing_measurements(&measurements).to_string();
    }

    fn solve_part2() -> String {
        let measurements = get_measurements();
        let windows = get_sliding_windows(&measurements, 3)
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .collect::<Vec<i32>>();
        return count_increasing_measurements(&windows).to_string();
    }
}
