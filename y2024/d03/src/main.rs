use aoc::error::Result;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

#[derive(Debug)]
enum Operator {
    Mul(i32, i32),
    Enable(),
    Disable(),
}

fn parse_input(input: &str) -> Result<Vec<Operator>> {
    let regex = regex::Regex::new(r"(mul\((\d+),(\d+)\)|don\'t\(\)|do\(\))")?;
    let mut operators = Vec::new();

    regex.find_iter(input).for_each(|cap| {
        let op = cap.as_str().split("(").next().unwrap();

        operators.push(match op {
            "mul" => {
                let digits = cap
                    .as_str()
                    .trim_start_matches("mul(")
                    .trim_end_matches(")")
                    .split(",")
                    .map(|d| d.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                Operator::Mul(digits[0], digits[1])
            }
            "do" => Operator::Enable(),
            "don't" => Operator::Disable(),
            _ => panic!("Unknown operator: {}", cap.as_str()),
        });
    });

    Ok(operators)
}

fn evaluate(operators: Vec<Operator>, ignore_disable: bool) -> i32 {
    let mut result = 0;
    let mut is_enabled = true;

    for op in operators {
        match op {
            Operator::Mul(a, b) => {
                if is_enabled || ignore_disable {
                    result += a * b;
                }
            }
            Operator::Enable() => is_enabled = true,
            Operator::Disable() => is_enabled = false,
        }
    }

    result
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let operators = parse_input(input)?;

        Ok(evaluate(operators, true) as usize)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let operators = parse_input(input)?;

        Ok(evaluate(operators, false) as usize)
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
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn sample_test() {
        let operators = parse_input(INPUT).unwrap();

        assert_eq!(operators.len(), 4);
    }
}
