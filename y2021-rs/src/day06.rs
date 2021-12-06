use std::collections::hash_map;
use std::collections::HashMap;

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

    // create initial hashmap
    let mut fish: HashMap<u8, usize> = hash_map::HashMap::new();
    let keys: Vec<u8> = (0..=8).collect();
    for key in &keys {
        fish.insert(
            key.clone(),
            (&initial_fish.iter().filter(|x| x == &key).count()).clone(),
        );
    }

    // simulate days
    for _ in 0..days {
        let mut next_fish: HashMap<u8, usize> = hash_map::HashMap::new();

        for i in 0..=8 {
            let n_fish = fish[&i];

            if i == 0 {
                *next_fish.entry(8).or_insert(0) += n_fish;
                *next_fish.entry(6).or_insert(0) += n_fish;
            } else {
                *next_fish.entry(i - 1).or_insert(0) += n_fish;
            }
        }

        fish = next_fish;
    }

    return fish.values().sum::<usize>().to_string();
}

impl day::Day for Day06 {
    fn solve_part1() -> String {
        simulate(80)
    }

    fn solve_part2() -> String {
        simulate(256)
    }
}
