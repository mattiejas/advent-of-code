use crate::day;
use crate::utils;

pub struct Day06 {}

fn get_initial_fish(input: &str) -> Vec<usize> {
    let initial_fish: Vec<u8> = input
        .trim()
        .split(',')
        .map(|x| x.to_owned().parse::<u8>())
        .flatten()
        .collect();

    let mut fish: Vec<usize> = vec![0; 9]; // move fish into age brackets
    for x in initial_fish {
        fish[x as usize] += 1;
    }
    return fish;
}

fn simulate(days: u16) -> String {
    let mut fish = get_initial_fish(&utils::input(6, false)); // comma separated fish

    for _ in 0..days {
        fish.rotate_left(1); // simulate days
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
