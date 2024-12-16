use aoc::{
    coord::{Coord, Direction},
    error::Result,
};
use itertools::Itertools;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

#[derive(Debug, Clone, Copy)]
enum TileType {
    Empty,
    Wall,
    Box,
}

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<TileType>,
}

#[derive(Debug, Clone)]
struct Robot {
    position: Coord<i32>,
    moves: Vec<Direction<i32>>,
}

fn parse(input: &str) -> (Map, Robot) {
    let (map_str, moves_str) = input.trim().split("\n\n").collect_tuple().unwrap();

    let width = map_str.lines().next().unwrap().len();
    let height = map_str.lines().count();
    let map = map_str
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '#' => TileType::Wall,
            'O' => TileType::Box,
            '@' => TileType::Empty,
            '.' => TileType::Empty,
            _ => panic!("Invalid tile type"),
        })
        .collect();

    // get position of robot
    // @ is the robot
    let robot_position = map_str
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .find(|(_, _, c)| *c == '@')
        .map(|(x, y, _)| (x, y))
        .unwrap();

    let moves = moves_str
        .trim()
        .replace('\n', "")
        .chars()
        .map(|c| match c {
            'v' => Direction { x: 0, y: 1 },
            '^' => Direction { x: 0, y: -1 },
            '<' => Direction { x: -1, y: 0 },
            '>' => Direction { x: 1, y: 0 },
            _ => panic!("Invalid move"),
        })
        .collect();

    let robot = Robot {
        position: Coord::new(robot_position.0 as i32, robot_position.1 as i32),
        moves,
    };

    (
        Map {
            width,
            height,
            tiles: map,
        },
        robot,
    )
}

impl Map {
    fn get_tile(&self, x: i32, y: i32) -> TileType {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            TileType::Wall
        } else {
            self.tiles[(y * self.width as i32 + x) as usize]
        }
    }

    fn box_find_next_wall_or_empty(
        &self,
        x: i32,
        y: i32,
        direction: Direction<i32>,
    ) -> (TileType, Coord<i32>) {
        let mut x = x;
        let mut y = y;

        loop {
            x += direction.x;
            y += direction.y;

            let tile = self.get_tile(x, y);

            match tile {
                TileType::Wall => return (TileType::Wall, Coord::new(x, y)),
                TileType::Box => continue,
                TileType::Empty => return (TileType::Empty, Coord::new(x, y)),
            }
        }
    }

    fn box_move(&mut self, x: i32, y: i32, direction: Direction<i32>) {
        let (next_type, next_position) = self.box_find_next_wall_or_empty(x, y, direction);

        match next_type {
            TileType::Empty => loop {
                let x = x + direction.x;
                let y = y + direction.y;
            },
        }
    }

    fn simulate(&mut self, robot: &mut Robot) {
        for direction in robot.moves.iter() {
            let next_position = robot.position.add(*direction);
            let next_tile = self.get_tile(next_position.x, next_position.y);

            match next_tile {
                TileType::Empty => {
                    robot.position = next_position;
                }
                TileType::Box => {
                    let (next_type, next_position) = self.box_find_next_wall_or_empty(
                        next_position.x,
                        next_position.y,
                        *direction,
                    );

                    if next_type == TileType::Empty {
                        robot.position = next_position;
                    }
                }
                TileType::Wall => {
                    // do nothing
                }
            }
        }
    }

    fn score(&self) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .map(|(i, t)| match t {
                TileType::Box => {
                    let x = i % self.width;
                    let y = i / self.width;

                    100 * y + x
                }
                _ => 0,
            })
            .sum()
    }

    fn draw(&self, robot: &Robot) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get_tile(x as i32, y as i32);

                if robot.position.x == x as i32 && robot.position.y == y as i32 {
                    print!("@");
                } else {
                    match tile {
                        TileType::Empty => print!("."),
                        TileType::Wall => print!("#"),
                        TileType::Box => print!("O"),
                    }
                }
            }

            println!();
        }
    }
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let (mut map, mut robot) = parse(input);

        map.simulate(&mut robot);
        map.draw(&robot);

        Ok(map.score())
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
    use aoc::Part;

    use super::*;

    const SAMPLE: &str = r#"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;

    #[test]
    fn part1_sample_test() {
        let part = Part1;
        let expected = 10092;

        let result = part.solve(SAMPLE).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn part2_sample_test() {
        assert_eq!(1, 1);
    }
}
