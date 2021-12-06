use std::collections::hash_map;

use crate::day;
use crate::utils;

pub struct Day06 {}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split('\n')
        .next()
        .unwrap()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn simulate(days: i64) -> String {
    let input = utils::input(6, false);
    let initial_fish = parse_input(&input);

    let mut fish = hash_map::HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ]);

    for i in 0..initial_fish.len() {
        let f = initial_fish[i];
        fish.insert(f, fish.get(&f).unwrap() + 1);
    }

    for _ in 0..days {
        let mut next_fish = hash_map::HashMap::new();

        for i in 0..=8 {
            let n_fish = fish[&i];

            if i == 0 {
                // create new fish
                if let Some(f) = next_fish.get(&8).cloned() {
                    next_fish.insert(8, f + n_fish);
                } else {
                    next_fish.insert(8, n_fish);
                }

                // update existing fish
                if let Some(f) = next_fish.get(&6).cloned() {
                    next_fish.insert(6, f + n_fish);
                } else {
                    next_fish.insert(6, n_fish);
                }
            } else {
                // update old fish
                if let Some(f) = next_fish.get(&(i - 1)).cloned() {
                    next_fish.insert(i - 1, f + n_fish);
                } else {
                    next_fish.insert(i - 1, n_fish);
                }
            }
        }

        fish = next_fish;
    }

    return fish.values().sum::<i64>().to_string();
}

impl day::Day for Day06 {
    fn solve_part1() -> String {
        simulate(80)
    }

    fn solve_part2() -> String {
        simulate(256)
    }
}
