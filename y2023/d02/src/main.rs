use std::collections::HashMap;
use aoc::error::*;
use log::{info, warn};

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let lines = aoc::split_input(input);

        let mut games = Vec::new();

        for line in lines {
            let game = Game::from_str(line)?;
            games.push(game);
        }

        let restrictions = vec![
            (Color::Red, 12),
            (Color::Green, 13),
            (Color::Blue, 14),
        ];

        let viable_games = games.iter().filter(|game| {
            for (color, count) in &restrictions {
                let game_color = game.colors.get(color).unwrap();
                if game_color > count {
                    return false;
                }
            }
            true
        }).collect::<Vec<_>>();

        let summed_ids = viable_games.iter().map(|game| game.id).sum::<usize>();

        Ok(summed_ids)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let lines = aoc::split_input(input);

        let mut games = Vec::new();

        for line in lines {
            let game = Game::from_str(line)?;
            games.push(game);
        }

        let power = games.iter()
            .map(
                |game|
                    game.colors.get(&Color::Red).unwrap() *
                        game.colors.get(&Color::Green).unwrap() *
                        game.colors.get(&Color::Blue).unwrap()
            )
            .sum::<usize>();

        Ok(power)
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

#[derive(Debug, Clone, Copy, Hash)]
#[derive(Eq, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn from_str(input: &str) -> aoc::error::Result<Self> {
        match input {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(AocError::ParseError(format!("Invalid color: {}", input))),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    colors: HashMap<Color, usize>,
}

impl Game {
    fn new(id: usize, red: usize, green: usize, blue: usize) -> Self {
        let mut colors = HashMap::new();
        colors.insert(Color::Red, red);
        colors.insert(Color::Green, green);
        colors.insert(Color::Blue, blue);

        Self {
            id,
            colors,
        }
    }

    fn from_str(input: &str) -> aoc::error::Result<Self> {
        let mut parts = input.split(": ");

        let id = parts.next().unwrap().trim_start_matches("Game ").parse::<usize>()?;
        let mut game = Game::new(id, 0, 0, 0);

        let mut sets = parts.next().unwrap().split("; ");

        for set in sets {
            let mut colors = set.split(", ");

            for color in colors {
                let count = color.split(" ").next().unwrap().parse::<usize>()?;
                let color = color.split(" ").last().unwrap();

                let color = Color::from_str(color)?;

                let game_color = game.colors.get_mut(&color).unwrap();

                if count > *game_color {
                    *game_color = count;
                }
            }
        }

        Ok(game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsed_correctly() {
        // Arrange
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game::new(1, 4, 2, 6);

        // Act
        let actual = Game::from_str(game).unwrap();

        // Assert
        assert_eq!(expected, actual);
    }
}
