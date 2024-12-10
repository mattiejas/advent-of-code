use aoc::{
    coord::{Coord, CARDINAL_DIRECTIONS},
    error::Result,
};

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Trailhead,
    Trail(i32),
    Top,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '0' => Tile::Trailhead,
            '9' => Tile::Top,
            '1'..='8' => Tile::Trail(c as i32 - '0' as i32),
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    trailheads: Vec<(usize, usize)>,
}

impl Map {
    fn from_input(input: &str) -> Self {
        let trimmed = input.trim();
        let mut tiles = Vec::new();
        let mut trailheads = Vec::new();
        let width = trimmed.lines().next().unwrap().len();
        let height = trimmed.lines().count();

        for (y, line) in trimmed.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile = Tile::from_char(c);
                tiles.push(tile);
                if let Tile::Trailhead = tile {
                    trailheads.push((x, y));
                }
            }
        }

        Self {
            tiles,
            trailheads,
            width,
            height,
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y * self.width + x]
    }

    fn get_score_for_trailhead(&self, x: usize, y: usize) -> i32 {
        let mut score = 0;
        let mut visited = Vec::<Coord<i32>>::new();
        let mut next_tiles = vec![Coord::new(x as i32, y as i32)];
        let directions: Vec<Coord<i32>> = CARDINAL_DIRECTIONS
            .iter()
            .map(|&d| Coord::from(d))
            .collect();

        while let Some(coord) = next_tiles.pop() {
            if visited.contains(&coord) {
                continue;
            }
            visited.push(coord);

            let tile = self.get_tile(coord.x as usize, coord.y as usize);
            let height = match tile {
                Tile::Trail(h) => *h,
                Tile::Trailhead => 0,
                Tile::Top => {
                    score += 1;
                    continue;
                }
            };

            for direction in &directions {
                let neighbor = coord + *direction;
                if neighbor.x < 0
                    || neighbor.x >= self.width as i32
                    || neighbor.y < 0
                    || neighbor.y >= self.height as i32
                {
                    continue;
                }

                if visited.contains(&neighbor) {
                    continue;
                }

                let neighbor_tile = self.get_tile(neighbor.x as usize, neighbor.y as usize);
                match neighbor_tile {
                    Tile::Trail(h) if *h - height == 1 => next_tiles.push(neighbor),
                    Tile::Top if height == 8 => next_tiles.push(neighbor),
                    _ => {}
                }
            }
        }

        score
    }

    fn get_rating_for_trailhead(&self, x: usize, y: usize) -> i32 {
        let mut unique_trails = std::collections::HashSet::new();
        let mut stack = vec![(Coord::new(x as i32, y as i32), vec![])];
        let directions: Vec<Coord<i32>> = CARDINAL_DIRECTIONS
            .iter()
            .map(|&d| Coord::from(d))
            .collect();

        while let Some((coord, path)) = stack.pop() {
            let tile = self.get_tile(coord.x as usize, coord.y as usize);
            let height = match tile {
                Tile::Trail(h) => *h,
                Tile::Trailhead => 0,
                Tile::Top => {
                    unique_trails.insert(path);
                    continue;
                }
            };

            for direction in &directions {
                let neighbor = coord + *direction;
                if neighbor.x < 0
                    || neighbor.x >= self.width as i32
                    || neighbor.y < 0
                    || neighbor.y >= self.height as i32
                {
                    continue;
                }

                if path.contains(&neighbor) {
                    continue;
                }

                let neighbor_tile = self.get_tile(neighbor.x as usize, neighbor.y as usize);
                match neighbor_tile {
                    Tile::Trail(h) if *h - height == 1 => {
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        stack.push((neighbor, new_path));
                    }
                    Tile::Top if height == 8 => {
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        stack.push((neighbor, new_path));
                    }
                    _ => {}
                }
            }
        }

        unique_trails.len() as i32
    }
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let map = Map::from_input(input);
        let total_score: i32 = map
            .trailheads
            .iter()
            .map(|&(x, y)| map.get_score_for_trailhead(x, y))
            .sum();
        Ok(total_score as usize)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let map = Map::from_input(input);
        let total_rating: i32 = map
            .trailheads
            .iter()
            .map(|&(x, y)| map.get_rating_for_trailhead(x, y))
            .sum();
        Ok(total_rating as usize)
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
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

    #[test]
    fn part1_sample_test() {
        let part1 = Part1;
        assert_eq!(part1.solve(SAMPLE).unwrap(), 36);
    }

    #[test]
    fn part2_sample_test() {
        let part2 = Part2;
        assert_eq!(part2.solve(SAMPLE).unwrap(), 81);
    }
}
