use aoc::error::Result;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut deltas = Vec::new();

    for i in 1..report.len() {
        deltas.push(report[i] - report[i - 1]);
    }

    // check if deltas are either all positive or all negative
    let all_positive = deltas.iter().all(|x| *x >= 1 && *x <= 3);
    let all_negative = deltas.iter().all(|x| *x >= -3 && *x <= -1);

    all_positive || all_negative
}

fn is_report_safe_optional_level(report: &Vec<i32>) -> bool {
    if is_report_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);

        if is_report_safe(&new_report) {
            return true;
        }
    }

    false
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .split("\n")
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let count = parse(input)
            .iter()
            .map(|report| {
                let is_safe = is_report_safe(report);

                if is_safe {
                    1
                } else {
                    0
                }
            })
            .sum();

        Ok(count)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let count = parse(input)
            .iter()
            .map(|report| {
                let is_safe = is_report_safe_optional_level(report);

                if is_safe {
                    1
                } else {
                    0
                }
            })
            .sum();

        Ok(count)
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

    #[test]
    fn sample_test() {
        assert_eq!(1, 1);
    }
}
