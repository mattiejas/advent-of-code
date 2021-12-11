use crate::{day, utils};

pub struct Day11 {}

fn get_map() -> Vec<Vec<i32>> {
    let input = utils::input(11, false);
    return input
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
}

impl day::Day for Day11 {
    fn solve_part1() -> String {
        let map = get_map();

        "".to_string()
    }

    fn solve_part2() -> String {
        "".to_string()
    }
}
