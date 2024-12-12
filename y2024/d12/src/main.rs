use std::collections::{HashMap, HashSet};

use aoc::{
    coord::{
        Coord, CARDINAL_DIRECTIONS, EAST, NORTH, NORTH_EAST, NORTH_WEST, SOUTH, SOUTH_EAST,
        SOUTH_WEST, WEST,
    },
    error::Result,
};

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

type RegionType = char;

#[derive(Debug)]
struct Map {
    map: Vec<RegionType>,
    width: usize,
    height: usize,
    distinct_regions: Vec<Region>,
}

#[derive(Debug)]
struct Region {
    region_type: RegionType,
    coords: Vec<Coord<i32>>,
}

fn parse(input: &str) -> Map {
    let width = input.trim().lines().next().unwrap().len();
    let height = input.trim().lines().count();

    let map = input.trim().lines().flat_map(|line| line.chars()).collect();

    Map {
        map,
        width,
        height,
        distinct_regions: Vec::new(),
    }
}

impl Map {
    fn get(&self, coord: Coord<i32>) -> RegionType {
        self.map[coord.y as usize * self.width + coord.x as usize]
    }

    fn find_distinct_regions(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord {
                    x: x as i32,
                    y: y as i32,
                };
                let region_type = self.get(coord);

                // check if region is already in a distinct region
                if self
                    .distinct_regions
                    .iter()
                    .any(|region| region.coords.contains(&coord))
                {
                    continue;
                }

                let mut region = Region {
                    region_type,
                    coords: Vec::new(),
                };

                self.flood_fill(coord, &mut region);
                self.distinct_regions.push(region);
            }
        }
    }

    fn flood_fill(&self, coord: Coord<i32>, region: &mut Region) {
        let mut queue = Vec::new();
        queue.push(coord);

        while let Some(coord) = queue.pop() {
            let region_type = self.get(coord);

            // check if region is already in a distinct region
            if region.coords.contains(&coord) {
                continue;
            }

            for direction in CARDINAL_DIRECTIONS {
                let neighbor = Coord::new(coord.x + direction.x, coord.y + direction.y);
                if neighbor.x < 0
                    || neighbor.x >= self.width as i32
                    || neighbor.y < 0
                    || neighbor.y >= self.height as i32
                {
                    continue;
                }

                if self.get(neighbor) != region_type {
                    continue;
                }

                queue.push(neighbor);
            }

            // add current region to distinct region
            region.coords.push(coord);
        }
    }
}

impl Region {
    fn perimeter(&self) -> Vec<Coord<i32>> {
        let mut perimeter = Vec::<Coord<i32>>::new();

        for coord in &self.coords {
            for direction in CARDINAL_DIRECTIONS {
                let neighbor = Coord::new(coord.x + direction.x, coord.y + direction.y);
                if !self.coords.contains(&neighbor) {
                    perimeter.push(*coord);
                }
            }
        }

        perimeter
    }

    fn sides(&self) -> usize {
        let mut corner_count = 0;

        for coord in &self.coords {
            // Concave
            let coordinates = [
                (*coord + NORTH, *coord + EAST),
                (*coord + NORTH, *coord + WEST),
                (*coord + SOUTH, *coord + EAST),
                (*coord + SOUTH, *coord + WEST),
            ];

            for (a, b) in coordinates {
                if !self.coords.contains(&a) && !self.coords.contains(&b) {
                    corner_count += 1;
                }
            }

            // Convex
            let coordinates = [
                (*coord + NORTH, *coord + EAST, *coord + NORTH_EAST),
                (*coord + NORTH, *coord + WEST, *coord + NORTH_WEST),
                (*coord + SOUTH, *coord + EAST, *coord + SOUTH_EAST),
                (*coord + SOUTH, *coord + WEST, *coord + SOUTH_WEST),
            ];

            for (a, b, c) in coordinates {
                if self.coords.contains(&a) && self.coords.contains(&b) && !self.coords.contains(&c)
                {
                    corner_count += 1;
                }
            }
        }

        corner_count
    }
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let mut map = parse(input);
        map.find_distinct_regions();

        Ok(map
            .distinct_regions
            .iter()
            .map(|region| region.perimeter().len() * region.coords.len())
            .sum())
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let mut map = parse(input);
        map.find_distinct_regions();

        Ok(map
            .distinct_regions
            .iter()
            .map(|region| region.sides() * region.coords.len())
            .sum())
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
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;

    #[test]
    fn part1_sample_test() {
        let part1 = Part1;
        assert_eq!(part1.solve(SAMPLE).unwrap(), 1930);
    }

    #[test]
    fn part2_sample_test() {
        let part2 = Part2;
        assert_eq!(part2.solve(SAMPLE).unwrap(), 1206);
    }
}
