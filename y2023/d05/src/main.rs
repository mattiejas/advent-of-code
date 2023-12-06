use std::collections::HashMap;
use indicatif::{ParallelProgressIterator};
use aoc::error::{Result};
use rayon::prelude::*;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

#[derive(Debug, Clone)]
struct Instruction {
    from: String,
    to: String,
    destination_range: (u64, u64),
    source_range: (u64, u64),
}

impl aoc::Part<&str, u64> for Part1 {
    fn solve(&self, input: &str) -> Result<u64> {
        let lines = aoc::split_input(input);
        let seeds = parse_seeds(lines[0])?;
        let instructions = parse_instructions(&lines[1..])?;
        let order = parse_instruction_order(&lines[1..])?;
        let lowest_location = find_lowest_number_in_range(&seeds, &order, &instructions);
        Ok(lowest_location)

    }
}

impl aoc::Part<&str, u64> for Part2 {
    fn solve(&self, input: &str) -> Result<u64> {
        let lines = aoc::split_input(input);
        let seeds = parse_seeds_ranges(lines[0])?;
        let instructions = parse_instructions(&lines[1..])?;
        let order = parse_instruction_order(&lines[1..])?;
        let lowest_location = find_lowest_number_in_range(&flatten_seeds(&seeds), &order, &instructions);
        Ok(lowest_location)
    }
}

fn find_lowest_number_in_range(seeds: &Vec<u64>, order: &[String], instructions: &HashMap<String, Vec<Instruction>>) -> u64 {
    let locations = seeds.par_iter().progress().map(|seed| {
        order[..order.len() - 1].iter().fold(*seed, |seed, from| {
           instructions.get(from).unwrap().iter().find(|instruction| {
               let source_range = instruction.source_range.0..instruction.source_range.1;
               source_range.contains(&seed)
           }).map(|instruction| {
               seed - instruction.source_range.0 + instruction.destination_range.0
           }).unwrap_or(seed)
        })
    }).collect::<Vec<u64>>();

    let min_location = locations.iter().min().unwrap();
    *min_location
}

fn parse_instruction_order(input: &[&str]) -> Result<Vec<String>> {
    let mut order: Vec<String> = Vec::new();

    let re = regex::Regex::new(r"(\w+)-to-(\w+)\s+map:")?;

    let mut from;
    let mut to = "";

    for line in input {
        match re.captures(line) {
            Some(captures) => {
                from = captures.get(1).unwrap().as_str();
                to = captures.get(2).unwrap().as_str();

                order.push(from.to_string());
            }
            None => {}
        }
    }

    order.push(to.to_string());
    Ok(order)
}

fn parse_seeds(input: &str) -> Result<Vec<u64>> {
    let seeds: Vec<u64>;

    match regex::Regex::new(r"seeds:\s+(.*)")?.captures(input) {
        Some(captures) => {
            let seed_str = captures.get(1).unwrap().as_str();
            seeds = seed_str.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
        }
        None => return Err(aoc::error::AocError::ParseError("Failed to parse seeds".to_string())),
    }

    Ok(seeds)
}

fn parse_seeds_ranges(input: &str) -> Result<Vec<(u64, u64)>> {
    let seeds: Vec<(u64, u64)>;

    match regex::Regex::new(r"seeds:\s+(.*)")?.captures(input) {
        Some(captures) => {
            let seed_str = captures.get(1).unwrap().as_str();
            let numbers = seed_str.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            seeds = numbers.chunks(2)
                .map(|pair| (pair[0], pair[0] + pair[1]))
                .collect::<Vec<(u64, u64)>>();
        }
        None => return Err(aoc::error::AocError::ParseError("Failed to parse seeds".to_string())),
    }

    Ok(seeds)
}

fn flatten_seeds(seeds: &Vec<(u64, u64)>) -> Vec<u64> {
    let mut flattened_seeds = Vec::new();

    for (start, end) in seeds {
        for seed in *start..*end {
            flattened_seeds.push(seed);
        }
    }

    flattened_seeds
}

fn parse_instructions(lines: &[&str]) -> Result<HashMap<String, Vec<Instruction>>> {
    let map_re = regex::Regex::new(r"(\w+)-to-(\w+)\s+map:")?;
    let values_re = regex::Regex::new(r"(\d+)\s+(\d+)\s+(\d+)")?;

    let mut next_from = "";
    let mut next_to = "";

    let mut instructions = Vec::new();

    for line in lines {
        // does this line match the map regex?
        let map_match = map_re.captures(line);

        match map_match {
            Some(captures) => {
                next_from = captures.get(1).unwrap().as_str();
                next_to = captures.get(2).unwrap().as_str();
                continue;
            }
            _ => {}
        }

        match values_re.captures(line) {
            Some(captures) => {
                let destination_start = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                let source_start = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
                let length = captures.get(3).unwrap().as_str().parse::<u64>().unwrap();

                instructions.push(Instruction {
                    from: next_from.to_string(),
                    to: next_to.to_string(),
                    destination_range: (destination_start, destination_start + length),
                    source_range: (source_start, source_start + length),
                });
            }
            _ => {}
        }
    }

    let instruction_map = instructions.iter().fold(HashMap::<String, Vec<Instruction>>::new(), |mut map, instruction| {
        let instructions = map.entry(instruction.from.clone()).or_insert(Vec::new());
        instructions.push(instruction.clone());

        map
    });

    Ok(instruction_map)
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
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 1 2 3 4 5";
        let seeds = parse_seeds(input).unwrap();

        assert_eq!(seeds, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_instructions() {
        let input = r"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        ";

        let lines = aoc::split_input(input);
        let seeds = parse_seeds(lines[0]).unwrap();
        let instructions = parse_instructions(&lines[1..]).unwrap();
        let order = parse_instruction_order(&lines[1..]).unwrap();

        assert_eq!(seeds, vec![79, 14, 55, 13]);

        let instruction = instructions.get("seed").unwrap().get(0).unwrap();
        assert_eq!(instruction.from, "seed");
        assert_eq!(instruction.to, "soil");
        assert_eq!(instruction.destination_range, (50, 52));
        assert_eq!(instruction.source_range, (98, 100));

        let location = find_location(79, 0, &order, &instructions).unwrap();
        assert_eq!(location, 82);
    }

    #[test]
    fn test_parse_instructions_with_ranges() {
        let input = r"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        ";

        let lines = aoc::split_input(input);
        let seeds = parse_seeds_ranges(lines[0]).unwrap();
        let instructions = parse_instructions(&lines[1..]).unwrap();
        let order = parse_instruction_order(&lines[1..]).unwrap();

        let instruction = instructions.get("seed").unwrap().get(0).unwrap();
        let flattened_seeds = flatten_seeds(&seeds);

        assert_eq!(flattened_seeds.len(), 27);
        assert_eq!(instruction.from, "seed");
        assert_eq!(instruction.to, "soil");
        assert_eq!(instruction.destination_range, (50, 52));
        assert_eq!(instruction.source_range, (98, 100));

        let min_location = find_lowest_number_in_range(&flattened_seeds, &order, &instructions);

        assert_eq!(min_location, 46);
    }
}
