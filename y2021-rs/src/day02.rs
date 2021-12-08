use crate::{day, utils};

enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Day02 {}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

type Command = (Direction, i32);

fn get_commands() -> Vec<Command> {
    let input = utils::input(2, false);
    return input
        .split('\n')
        .map(|line| {
            let mut tokens = line.split_whitespace();
            let direction = tokens.next().map(|s| match s {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!("Unknown direction: {}", s),
            });
            let distance = tokens.next().map(|s| s.parse::<i32>().unwrap());
            return (direction, distance);
        })
        .filter_map(|(direction, distance)| direction.zip(distance))
        .collect();
}

impl day::Day for Day02 {
    fn solve_part1() -> String {
        let mut pos = Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        };
        let commands = get_commands();

        for c in commands {
            match c.0 {
                Direction::Forward => {
                    pos.horizontal += c.1;
                }
                Direction::Down => {
                    pos.depth += c.1;
                }
                Direction::Up => {
                    pos.depth -= c.1;
                }
            }
        }

        return format!("{}", pos.horizontal * pos.depth);
    }

    fn solve_part2() -> String {
        let mut pos = Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        };
        let commands = get_commands();

        for c in commands {
            match c.0 {
                Direction::Forward => {
                    pos.horizontal += c.1;
                    pos.depth += pos.aim * c.1;
                }
                Direction::Down => {
                    pos.aim += c.1;
                }
                Direction::Up => {
                    pos.aim -= c.1;
                }
            }
        }

        return format!("{}", pos.horizontal * pos.depth);
    }
}
