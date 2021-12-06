use crate::day;
use crate::utils;

pub struct Day06 {}

fn parse_input(input: &str) -> Vec<u8> {
    input
        .split('\n')
        .next()
        .expect("No input")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u8>())
        .flatten()
        .collect()
}

fn simulate(days: u32) -> String {
    let input = utils::input(6, false);
    let initial_fish = parse_input(&input);

    let mut fish: Vec<usize> = (0..=8)
        .into_iter()
        .map(|key| {
            initial_fish
                .iter()
                .filter(|x| *x.clone() as usize == key)
                .count()
        })
        .collect();

    // simulate days
    for _ in 0..days {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }

    return fish.iter().sum::<usize>().to_string();
}

impl day::Day for Day06 {
    fn solve_part1() -> String {
        simulate(80)
    }

    fn solve_part2() -> String {
        simulate(256)
    }
}
