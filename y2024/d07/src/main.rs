use aoc::error::Result;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

enum Operation {
    Add,
    Multiply,
    Concat,
}

const PART1_OPERATIONS: [Operation; 2] = [Operation::Add, Operation::Multiply];
const PART2_OPERATIONS: [Operation; 3] = [Operation::Add, Operation::Multiply, Operation::Concat];

struct Equation {
    constants: Vec<u128>,
    result: u128,
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts.next().unwrap().parse().unwrap();
            let constants = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect();
            Equation { constants, result }
        })
        .collect()
}

fn solve(equation: &Equation, sum: u128, index: usize, operations: &[Operation]) -> bool {
    if index >= equation.constants.len() {
        return sum == equation.result;
    }

    if sum > equation.result {
        return false;
    }

    let next = equation.constants[index];
    for op in operations {
        match op {
            Operation::Add => {
                if solve(equation, sum + next, index + 1, operations) {
                    return true;
                }
            }
            Operation::Multiply => {
                let acc = if sum == 0 { next } else { sum * next };
                if solve(equation, acc, index + 1, operations) {
                    return true;
                }
            }
            Operation::Concat => {
                let next_sum = format!("{}{}", sum, next).parse().unwrap();

                if solve(equation, next_sum, index + 1, operations) {
                    return true;
                }
            }
        }
    }

    false
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let equations = parse(input);

        let sum: u128 = equations
            .iter()
            .map(|equation| {
                if solve(equation, 0, 0, &PART1_OPERATIONS) {
                    equation.result
                } else {
                    0
                }
            })
            .sum();

        Ok(sum as usize)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let equations = parse(input);

        let sum: u128 = equations
            .iter()
            .map(|equation| {
                if solve(equation, 0, 0, &PART2_OPERATIONS) {
                    equation.result
                } else {
                    0
                }
            })
            .sum();

        Ok(sum as usize)
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
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

    const SAMPLE2: &str = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
108: 4 5 3 9
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
108: 2 2 5 12
"#;

    #[test]
    fn sample_test_part1() {
        let part1 = Part1;

        assert_eq!(part1.solve(SAMPLE).unwrap(), 3749);
        assert_eq!(part1.solve(SAMPLE2).unwrap(), 3965);
    }

    #[test]
    fn sample_test_part2() {
        let part2 = Part2;

        assert_eq!(part2.solve(SAMPLE).unwrap(), 11387);
        assert_eq!(part2.solve(SAMPLE2).unwrap(), 11603);
    }
}
