use aoc::error::Result;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        Ok(0)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        Ok(0)
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

    const SAMPLE: &str = r#"
INSERT_SAMPLE_HERE
"#;

    #[test]
    fn part1_sample_test() {
        assert_eq!(1, 1);
    }

    #[test]
    fn part2_sample_test() {
        assert_eq!(1, 1);
    }
}
