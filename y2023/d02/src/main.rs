use aoc::error::*;

#[derive(Debug)]
struct Part1;
#[derive(Debug)]
struct Part2;

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        Ok(0)
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

struct Game {
    id: usize,
    red: usize,
    green: usize,
    blue: usize,
}

impl Game {
    fn new(id: usize, red: usize, green: usize, blue: usize) -> Self {
        Self { id, red, green, blue }
    }

    fn from_str(input: &str) -> aoc::error::Result<Self> {
        let mut parts = input.split(": ");

        let id = parts.next()?.trim_start_matches("Game ").parse::<usize>()?;
        let game = Game::new(id, 0, 0, 0);

        let mut sets = parts.next()?.split("; ");

        for set in sets {
            let mut colors = set.split(", ");

            for color in colors {
                let mut color_parts = color.split(" ");
                let count = color_parts.next()?.parse::<usize>()?;
                let color = color_parts.next()?;

                match color {
                    "red" => println!("red"),
                    "green" => println!("green"),
                    "blue" => println!("blue"),
                    _ => println!("unknown"),
                }
            }
        }

        Ok(Self::new(0, 0, 0, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsed_correctly() {
        // Arrange
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game::new(1, 1, 2, 6);

        // Act
        let actual = Game::from_str(game).unwrap();

        // Assert
        assert_eq!(expected, actual);
    }
}
