use aoc::error::{AocError, Result};

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

#[derive(Debug, Clone)]
struct Race {
    time_ms: usize,
    distance_mm: usize,
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let lines = aoc::split_input(input);
        Ok(0)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        Ok(0)
    }
}

impl Race {
    fn race(&self, charge_ms: usize) -> usize {
        let mut distance = 0;
        let mut speed = 0; // in mm per ms
        let mut elapsed_time_ms = 0;

        // timer
        while elapsed_time_ms < self.time_ms {
            if charge_ms <= elapsed_time_ms {
                speed += 1;
            } else {
                distance += speed;
            }

            // update elapsed time
            elapsed_time_ms += 1;
        }

        distance
    }

    fn from_str(line: &str) -> Result<Self> {
        let time_re = regex::Regex::new(r"Time: +(\d+)").unwrap();
        let distance_re = regex::Regex::new(r"Distance: +(\d+)").unwrap();

        let time_ms = time_re
            .captures(line)
            .ok_or_else(|| aoc::AocError::msg("invalid time"))?
            .get(1)
            .ok_or_else(|| aoc::AocError::msg("invalid time"))?
            .as_str()
            .parse::<usize>()?;

        let distance_mm = distance_re
            .captures(line)
            .ok_or_else(|| aoc::AocError::msg("invalid distance"))?
            .get(1)
            .ok_or_else(|| aoc::AocError::msg("invalid distance"))?
            .as_str()
            .parse::<usize>()?;

        Ok(Self {
            time_ms,
            distance_mm,
        })
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
Time:      7  15   30
Distance:  9  40  200";

        let lines = aoc::split_input(input);
        let race = Race::from_str(&lines[0]).unwrap();

        assert_eq!(lines.len(), 2);
        assert_eq!(race.time_ms, 7);
        assert_eq!(race.distance_mm, 9);
    }
}
