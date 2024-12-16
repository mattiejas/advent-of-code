use aoc::{
    coord::{Coord, Direction},
    error::Result,
};
use itertools::Itertools;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Empty,
    Wall,
    Box,
    BoxRight,
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
    println!("{}", input.trim());
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

    fn box_move(&mut self, x: i32, y: i32, direction: Direction<i32>) -> bool {
        let first_box = Coord::new(x, y);

        let mut x = x;
        let mut y = y;

        let mut boxes = Vec::new();
        boxes.push(first_box);

        loop {
            x += direction.x;
            y += direction.y;

            let tile = self.get_tile(x, y);

            match tile {
                TileType::Wall => return false,
                TileType::Box | TileType::BoxRight => {
                    boxes.push(Coord::new(x, y));
                }
                TileType::Empty => {
                    for i in (0..boxes.len()).rev() {
                        let curr = boxes[i];
                        let next = curr.add(direction);

                        self.tiles.swap(
                            (curr.y * self.width as i32 + curr.x) as usize,
                            (next.y * self.width as i32 + next.x) as usize,
                        );
                    }

                    return true;
                }
            }
        }
    }

    fn can_box_move_recursive(
        &mut self,
        x: i32,
        y: i32,
        direction: Direction<i32>,
        boxes: &mut Vec<Coord<i32>>,
    ) -> bool {
        let left = Coord::new(x + direction.x, y + direction.y);
        let right = Coord::new(left.x + 1, left.y);

        if boxes.contains(&left) {
            return true;
        }

        if match (
            self.get_tile(left.x, left.y),
            self.get_tile(right.x, right.y),
        ) {
            (TileType::Empty, TileType::Empty) => true,
            (TileType::Wall, _) | (_, TileType::Wall) => false,
            (TileType::Box, _) => self.can_box_move_recursive(left.x, left.y, direction, boxes),
            (TileType::Empty, TileType::Box) => {
                self.can_box_move_recursive(right.x, right.y, direction, boxes)
            }
            (TileType::BoxRight, TileType::Empty) => {
                self.can_box_move_recursive(left.x - 1, left.y, direction, boxes)
            }
            (TileType::BoxRight, TileType::Box) => {
                self.can_box_move_recursive(left.x - 1, left.y, direction, boxes)
                    && self.can_box_move_recursive(right.x, right.y, direction, boxes)
            }
            (a, b) => {
                self.draw(&Robot {
                    position: Coord::new(x, y),
                    moves: vec![direction],
                });
                unreachable!("Invalid tile combination: {:?}, {:?}", a, b)
            }
        } {
            boxes.push(left);
            return true;
        }

        false
    }

    fn simulate(&mut self, robot: &mut Robot) {
        for direction in robot.moves.iter() {
            let next_position = robot.position.add(*direction);
            let next_tile = self.get_tile(next_position.x, next_position.y);

            match next_tile {
                TileType::Empty => {
                    robot.position = next_position;
                }
                TileType::Box | TileType::BoxRight => {
                    if direction.y == 0 {
                        // horizontal
                        if self.box_move(next_position.x, next_position.y, *direction) {
                            robot.position = next_position;
                        }

                        continue;
                    }

                    // vertical
                    let mut boxes = Vec::new();
                    let left = if next_tile == TileType::Box {
                        Coord::new(next_position.x, next_position.y)
                    } else {
                        Coord::new(next_position.x - 1, next_position.y)
                    };

                    println!(
                        "Left: {:?}, tile: {:?}",
                        left,
                        self.get_tile(left.x, left.y)
                    );

                    if self.can_box_move_recursive(left.x, left.y, *direction, &mut boxes) {
                        println!("Can move boxes: {:?}", boxes);
                        // move boxes
                        for i in (0..boxes.len()).rev() {
                            let curr = boxes[i];
                            let next = curr.add(*direction);

                            self.tiles.swap(
                                (curr.y * self.width as i32 + curr.x) as usize,
                                (next.y * self.width as i32 + next.x) as usize,
                            );

                            // move the box to the right
                            self.tiles.swap(
                                (curr.y * self.width as i32 + (curr.x + 1)) as usize,
                                (next.y * self.width as i32 + (next.x + 1)) as usize,
                            );
                        }

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
                        TileType::Box => print!("["),
                        TileType::BoxRight => print!("]"),
                    }
                }
            }

            println!();
        }
    }

    fn extend_width(&mut self, robot: &mut Robot) {
        let new_width = self.width * 2;

        let mut new_tiles = vec![TileType::Empty; new_width * self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get_tile(x as i32, y as i32);

                match tile {
                    TileType::Empty => {
                        // set x and x + 1 to empty
                        new_tiles[(y * new_width + (x * 2)) as usize] = tile;
                        new_tiles[(y * new_width + (x * 2 + 1)) as usize] = tile;
                    }
                    TileType::Wall => {
                        // set x and x + 1 to wall
                        new_tiles[(y * new_width + (x * 2)) as usize] = tile;
                        new_tiles[(y * new_width + (x * 2 + 1)) as usize] = tile;
                    }
                    TileType::Box => {
                        // set x to box and x + 1 to empty
                        new_tiles[(y * new_width + (x * 2)) as usize] = TileType::Box;
                        new_tiles[(y * new_width + (x * 2 + 1)) as usize] = TileType::BoxRight;
                    }
                    TileType::BoxRight => unreachable!("BoxRight should not be in the map"),
                }
            }
        }

        self.width = new_width;
        self.tiles = new_tiles;

        robot.position.x *= 2;
    }
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let (mut map, mut robot) = parse(input);

        map.extend_width(&mut robot);
        map.draw(&robot);
        map.simulate(&mut robot);
        map.draw(&robot);

        Ok(map.score())
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let (mut map, mut robot) = parse(input);

        map.extend_width(&mut robot);
        map.draw(&robot);
        map.simulate(&mut robot);
        map.draw(&robot);

        Ok(map.score())
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
        let part = Part2;
        let expected = 9021;

        let result = part.solve(SAMPLE).unwrap();

        assert_eq!(result, expected);
    }
}
