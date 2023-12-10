use aoc::error::Result;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

#[derive(Debug)]
struct Measurements {
    values: Vec<i64>,
}

trait Differences {
    fn differences(&self) -> Vec<i64>;
}

impl Differences for Vec<i64> {
    fn differences(&self) -> Vec<i64> {
        let mut differences = Vec::new();

        for i in 0..self.len() - 1 {
            let difference = self[i + 1] - self[i];
            differences.push(difference);
        }

        differences
    }
}

impl Measurements {
    fn extrapolate(&self) -> i64 {
        let diff = self.values.differences();

        self._extrapolate(&diff) + self.values[self.values.len() - 1]
    }

    fn _extrapolate(&self, previous_deltas: &Vec<i64>) -> i64 {
        if previous_deltas.iter().filter(|d| **d != 0).count() == 0 {
            // return last element of previous_deltas
            return previous_deltas[previous_deltas.len() - 1];
        }

        let next_diff = previous_deltas.differences();

        let extra = self._extrapolate(&next_diff);

        // return last element of previous_deltas + extra
        previous_deltas[previous_deltas.len() - 1] + extra
    }
}

fn parse_measurements(input: &str) -> Vec<Measurements> {
    let lines = aoc::split_input(input);

    let mut measurements = Vec::new();

    for line in lines {
        let values = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        measurements.push(Measurements { values });
    }

    measurements
}

impl aoc::Part<&str, i64> for Part1 {
    fn solve(&self, input: &str) -> Result<i64> {
        let measurements = parse_measurements(input);

        let sum_extrapolated = measurements.iter().map(|m| m.extrapolate()).sum::<i64>();

        Ok(sum_extrapolated)
    }
}

impl aoc::Part<&str, i64> for Part2 {
    fn solve(&self, input: &str) -> Result<i64> {
        let measurements = parse_measurements(input);

        // reverse measurements
        let measurements = measurements
            .iter()
            .map(|m| {
                let mut values = m.values.clone();
                values.reverse();
                Measurements { values }
            })
            .collect::<Vec<Measurements>>();

        let sum_extrapolated = measurements.iter().map(|m| m.extrapolate()).sum::<i64>();

        Ok(sum_extrapolated)
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
        let input = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let measurements = parse_measurements(input);

        assert_eq!(measurements[0].extrapolate(), 18);
        assert_eq!(measurements[1].extrapolate(), 28);
        assert_eq!(measurements[2].extrapolate(), 68);
    }
}
