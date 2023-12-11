use aoc::{
    coord::{self, Coord},
    error::Result,
};

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

struct Map {
    width: isize,
    height: isize,
    start: coord::Coord<isize>,
    tiles: Vec<Tile>,
    distances: Vec<isize>,
}

impl Map {
    fn get_tile(&self, coord: Coord<isize>) -> Option<&Tile> {
        let width = self.width as isize;
        let height = self.height as isize;

        if coord.x < 0 || coord.x >= width || coord.y < 0 || coord.y >= height {
            return None;
        }

        let index = coord.y * width + coord.x;
        let tile = &self.tiles[index as usize];

        Some(tile)
    }

    fn get_neighbours(&self, coord: Coord<isize>) -> Vec<Coord<isize>> {
        let mut neighbours = Vec::new();

        let north = Coord::new(coord.x, coord.y - 1);
        let south = Coord::new(coord.x, coord.y + 1);
        let east = Coord::new(coord.x + 1, coord.y);
        let west = Coord::new(coord.x - 1, coord.y);

        let tile = match self.get_tile(coord) {
            Some(tile) => tile,
            None => return neighbours,
        };

        match tile {
            Tile::Vertical => {
                neighbours.push(north);
                neighbours.push(south);
            }
            Tile::Horizontal => {
                neighbours.push(east);
                neighbours.push(west);
            }
            Tile::NorthEast => {
                neighbours.push(north);
                neighbours.push(east);
            }
            Tile::NorthWest => {
                neighbours.push(north);
                neighbours.push(west);
            }
            Tile::SouthEast => {
                neighbours.push(south);
                neighbours.push(east);
            }
            Tile::SouthWest => {
                neighbours.push(south);
                neighbours.push(west);
            }
            Tile::Start => {
                neighbours.push(north);
                neighbours.push(south);
                neighbours.push(east);
                neighbours.push(west);
            }
            Tile::Ground => {}
        }

        neighbours
    }

    fn find_closing_loop(&self) -> Vec<Coord<isize>> {
        let mut undiscovered = vec![self.start];
        let mut discovered = Vec::new();

        while !undiscovered.is_empty() {
            let current = undiscovered.pop().unwrap();
            discovered.push(current);

            let neighbours = self.get_neighbours(current);

            for neighbour in neighbours {
                if discovered.contains(&neighbour) {
                    continue;
                }

                if undiscovered.contains(&neighbour) {
                    continue;
                }

                undiscovered.push(neighbour);
            }
        }

        discovered
    }

    fn remove_unreachable(&mut self) {
        let closing_loop = self.find_closing_loop();

        let mut tiles = self.tiles.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord::new(x, y);
                if closing_loop.contains(&coord) {
                    continue;
                }

                let index = y * self.width + x;
                tiles[index as usize] = Tile::Ground;
            }
        }

        self.tiles = tiles;
    }

    fn get_distance_to_furthest_point(&self) -> isize {
        let closing_loop = self.find_closing_loop();

        let middle = closing_loop.len() as f64 / 2.0;

        middle.ceil() as isize
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let tile = &self.tiles[index as usize];

                let c = match tile {
                    Tile::Vertical => '|',
                    Tile::Horizontal => '─',
                    Tile::NorthEast => '└',
                    Tile::NorthWest => '┘',
                    Tile::SouthWest => '┐',
                    Tile::SouthEast => '┌',
                    Tile::Ground => ' ',
                    Tile::Start => '@',
                };
                output.push(c);
            }

            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

fn parse_map(input: &str) -> Map {
    let lines = aoc::split_input(input);

    let width = lines[0].len();
    let height = lines.len();

    let mut tiles = Vec::new();
    let mut start: Coord<isize> = Coord::new(0, 0);

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '|' => tiles.push(Tile::Vertical),
                '-' => tiles.push(Tile::Horizontal),
                'L' => tiles.push(Tile::NorthEast),
                'J' => tiles.push(Tile::NorthWest),
                '7' => tiles.push(Tile::SouthWest),
                'F' => tiles.push(Tile::SouthEast),
                '.' => tiles.push(Tile::Ground),
                'S' => {
                    tiles.push(Tile::Start);
                    start = Coord::new(x as isize, y as isize);
                }
                _ => {}
            }
        }
    }

    Map {
        width: width.try_into().unwrap(),
        height: height.try_into().unwrap(),
        tiles,
        start,
        distances: Vec::new(),
    }
}

impl aoc::Part<&str, isize> for Part1 {
    fn solve(&self, input: &str) -> Result<isize> {
        let mut map = parse_map(input);
        map.remove_unreachable();

        print!("{}", map);

        Ok(map.get_distance_to_furthest_point())
    }
}

impl aoc::Part<&str, isize> for Part2 {
    fn solve(&self, input: &str) -> Result<isize> {
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

    #[test]
    fn sample_test() {
        let input = "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        let mut map = parse_map(input);
        println!("{}", map);

        assert_eq!(map.width, 5);
        assert_eq!(map.height, 5);
        assert_eq!(map.start, Coord::new(0, 2));
        assert_eq!(map.tiles.len(), 25);

        println!("{}", map);

        let closing_loop = map.find_closing_loop();
        println!("{:?}", closing_loop);

        map.remove_unreachable();

        println!("{}", map);

        assert_eq!(map.get_distance_to_furthest_point(), 8);
    }
}
