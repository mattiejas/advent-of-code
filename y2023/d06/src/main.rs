use std::f32::consts::E;

use aoc::error::{AocError, Result};
use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

#[derive(Debug, Clone)]
struct Race {
    time_ms: usize,
    distance_mm: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RaceResult {
    Win,
    Loss,
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let races = parse_races(input);

        match races {
            Ok(races) => return Ok(race_wins_multiplied(races.as_slice())),
            Err(e) => {
                panic!("err: {}", e);
            }
        }
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let races = parse_races(input).unwrap();

        let (time, distance) =
            races
                .iter()
                .fold(("".to_owned(), "".to_owned()), |(time, distance), race| {
                    return (
                        time.to_owned() + race.time_ms.to_string().as_str(),
                        distance.to_owned() + race.distance_mm.to_string().as_str(),
                    );
                });

        let race = Race {
            time_ms: time.parse::<usize>().unwrap(),
            distance_mm: distance.parse::<usize>().unwrap(),
        };

        return Ok(count_race_wins(&race));
    }
}

impl Race {
    fn race(&self, charge_ms: usize) -> RaceResult {
        let speed = charge_ms;
        let remaining_time = self.time_ms - charge_ms;

        if remaining_time * speed > self.distance_mm {
            return RaceResult::Win;
        } else {
            return RaceResult::Loss;
        }
    }
}

fn count_race_wins(race: &Race) -> usize {
    (0..race.time_ms)
        .into_par_iter()
        .map(|charge_time| race.race(charge_time))
        .filter(|result| *result == RaceResult::Win)
        .count()
}

fn race_wins_multiplied(races: &[Race]) -> usize {
    let mut product = 1;

    for race in races.iter().progress() {
        product *= count_race_wins(race);
    }

    product
}

fn parse_races(input: &str) -> Result<Vec<Race>> {
    let time_re = regex::Regex::new(r"Time:\s+(.*)").unwrap();
    let distance_re = regex::Regex::new(r"Distance:\s+(.*)").unwrap();

    let times = time_re
        .captures(input)
        .expect("time_re should match")
        .get(1)
        .expect("should have a capture")
        .as_str()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let distances = distance_re
        .captures(input)
        .expect("distance_re should match")
        .get(1)
        .expect("should have a capture")
        .as_str()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    if times.len() != distances.len() {
        return Err(AocError::ParseError(
            "times and distances must be the same length".to_owned(),
        ));
    }

    let races = times
        .iter()
        .zip(distances)
        .map(|(time, distance)| Race {
            time_ms: *time,
            distance_mm: distance,
        })
        .collect::<Vec<Race>>();

    Ok(races)
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
        let races = parse_races(input).unwrap();
        let race = races.get(0).unwrap();

        let win = race.race(2);

        assert_eq!(lines.len(), 2);
        assert_eq!(races.len(), 3);

        assert_eq!(win, RaceResult::Win);

        assert_eq!(race.time_ms, 7);
        assert_eq!(race.distance_mm, 9);
    }
}
