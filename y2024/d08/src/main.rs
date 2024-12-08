use std::collections::HashMap;

use aoc::{coord::Coord, error::Result};
use itertools::Itertools;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

fn parse(input: &str) -> HashMap<String, Vec<Coord<i64>>> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' => None,
                _ => Some((c.to_string(), Coord::new(x as i64, y as i64))),
            })
        })
        .flatten()
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).or_insert_with(Vec::new).push(v);
            acc
        })
}

fn find_antinotes(frequency: &str, nodes: &[Coord<i64>]) -> Vec<Coord<i64>> {
    let mut antinotes = Vec::new();

    // get all combinations of nodes
    for n in nodes.iter().combinations(2) {
        let a = n[0];
        let b = n[1];
        let direction = a.direction_to(b);

        let antinode_a = a.sub(direction);
        let antinode_b = b.add(direction);

        antinotes.push(antinode_a);
        antinotes.push(antinode_b);
    }

    antinotes
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let map = parse(input);
        let (min, max) = find_bounds(input);

        let mut antinotes: Vec<Coord<i64>> = Vec::new();

        for (frequency, nodes) in map.iter() {
            antinotes.extend(
                find_antinotes(frequency, nodes)
                    .iter()
                    .filter(|a| within_bounds(**a, min, max))
                    .copied()
                    .collect::<Vec<Coord<i64>>>(),
            );
        }

        // get unique antinotes
        let antinotes = antinotes.into_iter().unique().collect::<Vec<Coord<i64>>>();

        println!("{}", draw(&map, &antinotes, min, max));

        Ok(antinotes.len())
    }
}

fn within_bounds(a: Coord<i64>, min: Coord<i64>, max: Coord<i64>) -> bool {
    a.x >= min.x && a.x < max.x && a.y >= min.y && a.y < max.y
}

fn find_bounds(input: &str) -> (Coord<i64>, Coord<i64>) {
    let y = input.trim().lines().count() as i64;
    let x = input.trim().lines().next().unwrap().len() as i64;

    (Coord::new(0, 0), Coord::new(x, y))
}

fn draw(
    map: &HashMap<String, Vec<Coord<i64>>>,
    antinodes: &[Coord<i64>],
    min: Coord<i64>,
    max: Coord<i64>,
) -> String {
    // draw the character of the frequency, and # for antinodes
    let mut grid = vec![vec!['.'; (max.x - min.x) as usize]; (max.y - min.y) as usize];

    for (frequency, nodes) in map.iter() {
        for node in nodes {
            grid[node.y as usize][node.x as usize] = frequency.chars().next().unwrap();
        }
    }

    for antinode in antinodes {
        grid[antinode.y as usize][antinode.x as usize] = '#';
    }

    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n")
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
    use aoc::Part;

    use super::*;

    const SAMPLE: &str = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

    #[test]
    fn sample_test() {
        let part1 = Part1;

        assert_eq!(part1.solve(SAMPLE).unwrap(), 14);
    }
}
