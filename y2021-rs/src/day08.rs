use std::collections::HashMap;
use std::collections::HashSet;

use crate::day;
use crate::utils;

pub struct Day08;

fn get_input() -> Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> {
    return utils::input(8, false)
        .trim()
        .split('\n')
        .map(|x| {
            x.split_once('|').map(|(x, y)| {
                (
                    x.split_whitespace().map(|x| x.chars().collect()).collect(),
                    y.split_whitespace().map(|x| x.chars().collect()).collect(),
                )
            })
        })
        .flatten()
        .collect();
}

fn join(x: &HashSet<char>) -> String {
    let mut sorted = x.clone().into_iter().collect::<Vec<char>>();
    sorted.sort();
    return sorted.iter().collect();
}

impl day::Day for Day08 {
    fn solve_part1() -> String {
        let segments = get_input();
        let unique_lengths = vec![2, 3, 4, 7];

        return segments
            .iter()
            .map(|(_, x)| {
                x.iter()
                    .filter(|y| unique_lengths.contains(&y.len()))
                    .count()
            })
            .sum::<usize>()
            .to_string();
    }

    fn solve_part2() -> String {
        let lines = get_input();
        let mut results: Vec<u32> = vec![];

        for (s, o) in &lines {
            let four = s.iter().find(|x| x.len() == 4).unwrap();
            let seven = s.iter().find(|x| x.len() == 3).unwrap();
            let eight = s.iter().find(|x| x.len() == 7).unwrap();

            let config: HashMap<String, u8> = s
                .iter()
                .map(|x| match x.len() {
                    2 => (join(x), 1),
                    3 => (join(x), 7),
                    4 => (join(x), 4),
                    7 => (join(x), 8),
                    6 => {
                        if four.intersection(x).eq(four) {
                            return (join(x), 9);
                        }
                        if join(&x.union(&seven).cloned().collect()) == join(eight) {
                            return (join(x), 6);
                        }
                        return (join(x), 0);
                    }
                    5 => {
                        if seven.intersection(x).eq(seven) {
                            return (join(x), 3);
                        }
                        if join(&x.union(&four).cloned().collect()) == join(eight) {
                            return (join(x), 2);
                        }
                        return (join(x), 5);
                    }
                    _ => panic!("unexpected length"),
                })
                .collect();

            let numbers: Vec<u8> = o.iter().map(|x| config[&join(x)]).collect();

            let mut result = String::new();
            for x in numbers {
                result.push_str(&x.to_string());
            }
            results.push(result.parse::<u32>().unwrap());
        }

        return results.iter().sum::<u32>().to_string();
    }
}
