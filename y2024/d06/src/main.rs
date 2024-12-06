use std::collections::{HashMap, HashSet};

use aoc::error::{AocError, Result};
use rayon::collections::hash_map;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

fn parse_coordinated(input: &str) -> Result<Vec<(usize, usize)>> {
    let lines = input.trim().lines().collect::<Vec<&str>>();

    let coordinates = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(|(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .flat_map(|v| v)
        .collect::<Vec<(usize, usize)>>();

    Ok(coordinates)
}

fn find_guard(input: &str) -> Result<(usize, usize)> {
    let lines = input.trim().lines().collect::<Vec<&str>>();

    let guard = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains('^'))
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'^')
                .map(|(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .flat_map(|v| v)
        .collect::<Vec<(usize, usize)>>()[0];

    Ok(guard)
}

fn find_bounds(input: &str) -> Result<(usize, usize)> {
    let lines = input.trim().lines().collect::<Vec<&str>>();

    let width = lines[0].len();
    let height = lines.len();

    Ok((width, height))
}

fn simulate(
    guard: (usize, usize),
    walls: &Vec<(usize, usize)>,
    bounds: (usize, usize),
    limit: usize,
) -> Result<usize> {
    let mut visited = HashSet::new();

    let mut direction = (0, -1);
    let mut position = guard;
    let mut steps = 0;

    // keep walking until we reach a wall coordinate
    while position.0 >= 0 && position.0 < bounds.0 && position.1 >= 0 && position.1 < bounds.1 {
        if steps >= limit {
            return Err(AocError::ComputeError("Limit reached".to_string()));
        }

        visited.insert(position);
        steps += 1;

        let next = (
            position.0 as isize + direction.0,
            position.1 as isize + direction.1,
        );

        if walls.contains(&(next.0 as usize, next.1 as usize)) {
            // turn right
            direction = (-direction.1, direction.0);
        } else {
            position = (next.0 as usize, next.1 as usize);
        }
    }

    Ok(visited.len())
}

fn find_loop(
    guard: (usize, usize),
    walls: &Vec<(usize, usize)>,
    bounds: (usize, usize),
    limit: usize,
) -> Result<usize> {
    let mut count = 0;

    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            let mut walls_copy = walls.clone();
            walls_copy.push((x, y));

            let result = simulate(guard, &walls_copy, bounds, limit);

            if !result.is_ok() {
                count += 1;
            }
        }
    }

    Ok(count)
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let coordinates = parse_coordinated(input)?;
        let guard = find_guard(input)?;
        let bounds = find_bounds(input)?;

        Ok(simulate(guard, &coordinates, bounds, 10000)?)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let coordinates = parse_coordinated(input)?;
        let guard = find_guard(input)?;
        let bounds = find_bounds(input)?;

        Ok(find_loop(guard, &coordinates, bounds, 10000)?)
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
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    #[test]
    fn sample_test() {
        let part1 = Part1;

        assert_eq!(part1.solve(SAMPLE).unwrap(), 41);
    }

    #[test]
    fn sample_test2() {
        let part2 = Part2;

        assert_eq!(part2.solve(SAMPLE).unwrap(), 6);
    }
}
