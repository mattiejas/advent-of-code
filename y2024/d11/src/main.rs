use std::collections::HashMap;

use aoc::error::Result;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

fn parse_stones(input: &str) -> Vec<u128> {
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn insert_or_update(map: &mut HashMap<u128, usize>, key: u128, value: usize) {
    map.entry(key).and_modify(|v| *v += value).or_insert(value);
}

fn update_stones(initial_stones: HashMap<u128, usize>) -> HashMap<u128, usize> {
    let mut stones = HashMap::with_capacity(initial_stones.len());

    for (stone, count) in initial_stones {
        match stone {
            0 => {
                insert_or_update(&mut stones, 1, count);
            }
            stone if stone.to_string().len() % 2 == 0 => {
                let str = stone.to_string();
                let mut chars = str.chars();

                let first_half = chars
                    .by_ref()
                    .take(str.len() / 2)
                    .collect::<String>()
                    .parse()
                    .unwrap();

                let second_half = chars.collect::<String>().parse().unwrap();

                insert_or_update(&mut stones, first_half, count);
                insert_or_update(&mut stones, second_half, count);
            }
            stone => {
                insert_or_update(&mut stones, stone * 2024, count);
            }
        }
    }

    stones
}

fn evaluate(initial_stones: &[u128], blinks: usize) -> usize {
    let mut stones: HashMap<u128, usize> = HashMap::with_capacity(initial_stones.len());

    for stone in initial_stones {
        stones.entry(*stone).and_modify(|c| *c += 1).or_insert(1);
    }

    let bar = indicatif::ProgressBar::new(blinks as u64);
    for _ in 0..blinks {
        bar.inc(1);
        stones = update_stones(stones);
    }
    bar.finish();

    stones.values().sum()
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let stones = parse_stones(input);
        Ok(evaluate(&stones, 25))
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let stones = parse_stones(input);
        Ok(evaluate(&stones, 75))
    }
}

fn main() {
    aoc::init_logging();

    let input = include_str!("../input.txt");
    let solution = aoc::Solution::new(input.to_string());

    let part1 = Part1;
    solution.run(&part1);

    let part2 = Part2;
    solution.run(&part2);
}

#[cfg(test)]
mod tests {
    use aoc::Part;

    use super::*;

    const SAMPLE: &str = r#"
125 17
"#;

    #[test]
    fn part1_sample_test() {
        let part1 = Part1;
        assert_eq!(part1.solve(SAMPLE).unwrap(), 55312);
    }
}
