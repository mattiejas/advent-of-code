use aoc::coord::{BoundingBox, Coord};
use aoc::error::Result;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let lines = aoc::split_input(input);

        let sum = sum_nums_surrounding_symbols(&lines, |group, _| group.iter().sum())?;

        Ok(sum as usize)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let lines = aoc::split_input(input);

        let sum = sum_nums_surrounding_symbols(&lines, |group, char| {
            if char == '*' && group.len() > 1 {
                return group.iter().product();
            }

            0
        })?;

        Ok(sum as usize)
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

fn sum_nums_surrounding_symbols(lines: &[&str], combine: fn(Vec<i32>, char) -> i32) -> Result<i32> {
    let mut sum = 0;
    let symbols = find_special_symbols(&lines);
    let numbers = find_numbers(&lines);

    if lines.len() == 0 {
        return Ok(sum);
    }

    let max_line_len = lines[0].len();
    let max_lines = lines.len();

    for (symbol, symbol_coord) in &symbols {
        let mut nums_in_reach = Vec::new();

        for (num, num_coord) in &numbers {
            let bb = get_bounding_box_for_number(num_coord, *num, max_line_len, max_lines);

            if bb.contains(&symbol_coord) {
                nums_in_reach.push(*num);
            }
        }

        let result = combine(nums_in_reach, *symbol);
        sum += result;
    }

    Ok(sum)
}

fn find_numbers(lines: &[&str]) -> Vec<(i32, Coord<i32>)> {
    let mut numbers = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        numbers.append(&mut find_numbers_in_line(line, y as i32));
    }

    numbers
}

fn find_numbers_in_line(line: &str, y: i32) -> Vec<(i32, Coord<i32>)> {
    let mut numbers = Vec::new();

    let capture = regex::Regex::new(r"\d+").unwrap();

    for found_match in capture.find_iter(line) {
        let number = found_match.as_str().parse::<i32>().unwrap();
        numbers.push((number, Coord::<i32>::new(found_match.start() as i32, y)));
    }

    numbers
}

fn find_special_symbols(lines: &[&str]) -> Vec<(char, Coord<i32>)> {
    let mut symbols = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        symbols.append(&mut find_special_symbols_in_line(line, y as i32));
    }

    symbols
}

fn find_special_symbols_in_line(line: &str, y: i32) -> Vec<(char, Coord<i32>)> {
    let mut symbols = Vec::new();

    for (x, c) in line.chars().enumerate() {
        match c {
            '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => (),
            _ => symbols.push((c, Coord::<i32>::new(x as i32, y))),
        }
    }

    symbols
}

fn get_bounding_box_for_number(
    coord: &Coord<i32>,
    num: i32,
    max_line_len: usize,
    max_lines: usize,
) -> BoundingBox<i32> {
    let mut start = coord.clone();
    let mut end = coord.clone();

    let num_length = num.to_string().len() as i32;

    // Expand the bounding box to include a potential symbol to the left
    if start.x > 0 {
        start.x -= 1;
    }

    // Expand the bounding box to include a potential symbol above
    if start.y > 0 {
        start.y -= 1;
    }

    // Expand the bounding box to include a potential symbol to the right
    if end.x + num_length <= max_line_len as i32 {
        end.x += num_length;
    }

    // Expand the bounding box to include a potential symbol below
    if end.y + 1 <= max_lines as i32 {
        end.y += 1;
    }

    BoundingBox::new(start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_two_special_symbols() {
        // Arrange
        let input = "...$.*....";

        // Act
        let symbols = find_special_symbols_in_line(input, 0);

        // Assert
        assert_eq!(symbols.len(), 2);
    }

    #[test]
    fn find_coordinates_of_special_symbols() {
        // Arrange
        let input = r"
...*......
......#...
.....+.58.
...$.*....
.664.598..
        ";
        let lines = aoc::split_input(input);

        // Act
        let symbols = find_special_symbols(&lines);

        // Assert
        assert_eq!(symbols.len(), 5);
        assert_eq!(symbols[0], ('*', Coord::<i32>::new(3, 0)));
        assert_eq!(symbols[1], ('#', Coord::<i32>::new(6, 1)));
        assert_eq!(symbols[2], ('+', Coord::<i32>::new(5, 2)));
        assert_eq!(symbols[3], ('$', Coord::<i32>::new(3, 3)));
        assert_eq!(symbols[4], ('*', Coord::<i32>::new(5, 3)));
    }

    #[test]
    fn find_coordinates_of_numbers() {
        // Arrange
        let input = "123...456...";

        // Act
        let numbers = find_numbers_in_line(input, 0);

        // Assert
        assert_eq!(numbers.len(), 2);
        assert_eq!(numbers[0], (123, Coord::<i32>::new(0, 0)));
        assert_eq!(numbers[1], (456, Coord::<i32>::new(6, 0)));
    }

    #[test]
    fn find_coordinates_of_numbers_in_multiple_lines() {
        // Arrange
        let input = r"
123...456...
...789...101
        ";
        let lines = aoc::split_input(input);

        // Act
        let numbers = find_numbers(&lines);

        // Assert
        assert_eq!(numbers.len(), 4);
        assert_eq!(numbers[0], (123, Coord::<i32>::new(0, 0)));
        assert_eq!(numbers[1], (456, Coord::<i32>::new(6, 0)));
        assert_eq!(numbers[2], (789, Coord::<i32>::new(3, 1)));
        assert_eq!(numbers[3], (101, Coord::<i32>::new(9, 1)));
    }

    #[test]
    fn test_bounding_box_for_number() {
        // Arrange
        let coord = Coord::<i32>::new(5, 5);
        let num = 123;
        let max_line_len = 10;
        let max_lines = 10;

        // Act
        let bb = get_bounding_box_for_number(&coord, num, max_line_len, max_lines);

        // Assert
        assert_eq!(bb.start, Coord::<i32>::new(4, 4));
        assert_eq!(bb.end, Coord::<i32>::new(8, 6));
    }

    #[test]
    fn test_sample_input() {
        // Arrange
        let input = r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let lines = aoc::split_input(input);

        // Act
        let sum = sum_nums_surrounding_symbols(&lines, |group, _| group.iter().sum()).unwrap();

        // Assert
        assert_eq!(sum, 4361);
    }
}
