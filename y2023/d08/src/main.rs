use std::collections::HashMap;

use aoc::error::Result;
use rayon::prelude::*;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let pouch = CamelPouch::new(input);
        pouch.navigate("AAA", vec!["ZZZ".to_string()])
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let pouch = CamelPouch::new(input);
        pouch.navigate_to_trailing_z()
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

#[derive(Debug, Clone)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct CamelPouch {
    instructions: Vec<Instruction>,
    network_nodes: HashMap<String, (String, String)>,
}

impl CamelPouch {
    fn new(input: &str) -> Self {
        let lines = aoc::split_input(input);

        let instructions = lines[0]
            .trim()
            .chars()
            .map(|c| match c {
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => panic!("Invalid instruction: {}", c),
            })
            .collect::<Vec<_>>();

        // AAA = (BBB, CCC)
        let mapping_re = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        let mappings = lines[1..]
            .par_iter()
            .map(|line| {
                let captures = mapping_re.captures(line).unwrap();
                let from = captures.get(1).unwrap().as_str();
                let to1 = captures.get(2).unwrap().as_str();
                let to2 = captures.get(3).unwrap().as_str();
                (from, (to1, to2))
            })
            .collect::<Vec<_>>();

        let mut network_nodes = HashMap::new();

        for (from, (to1, to2)) in mappings {
            network_nodes.insert(from.to_string(), (to1.to_string(), to2.to_string()));
        }

        CamelPouch {
            instructions,
            network_nodes,
        }
    }

    fn navigate(&self, current_node: &str, target_nodes: Vec<String>) -> Result<usize> {
        let mut current_node = current_node;
        let mut i = 0;

        let mut current_instruction = &self.instructions[0];

        loop {
            let (left, right) = self.network_nodes.get(current_node).expect("Invalid node");

            match current_instruction {
                Instruction::Left => {
                    current_node = left;
                }
                Instruction::Right => {
                    current_node = right;
                }
            }

            i += 1;

            if target_nodes.contains(&current_node.to_string()) {
                return Ok(i);
            }

            current_instruction = &self.instructions[i % self.instructions.len()];
        }
    }

    fn navigate_to_trailing_z(&self) -> Result<usize> {
        // get the nodes that end with an 'A'
        let starting_nodes = self
            .network_nodes
            .par_iter()
            .filter(|(node, _)| node.ends_with('A'))
            .map(|(node, _)| node)
            .collect::<Vec<_>>();

        let target_nodes = self
            .network_nodes
            .par_iter()
            .filter(|(node, _)| node.ends_with('Z'))
            .map(|(node, _)| node.to_string())
            .collect::<Vec<_>>();

        let steps_per_node = starting_nodes
            .par_iter()
            .map(|node| self.navigate(node, target_nodes.clone()).unwrap())
            .collect::<Vec<_>>();

        return Ok(aoc::math::least_common_multiple(steps_per_node));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn sample_test() {
        let pouch = CamelPouch::new(SAMPLE_INPUT);
        let result = pouch.navigate("AAA", vec!["ZZZ".to_string()]).unwrap();

        assert_eq!(result, 6);
    }
}
