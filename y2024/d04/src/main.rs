use aoc::error::Result;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

fn find_horizontal(lines: &Vec<&str>, x: usize, y: usize) -> usize {
    let mut count = 0;
    let horizontal = lines[y];
    let substring = &horizontal[x..];

    if substring.starts_with("XMAS") {
        count += 1;
    }

    if substring.starts_with("SAMX") {
        count += 1;
    }

    count
}

fn find_vertical(lines: &Vec<&str>, x: usize, y: usize) -> usize {
    let mut count = 0;
    let vertical = lines
        .iter()
        .map(|l| l.chars().nth(x).unwrap())
        .collect::<String>();
    let substring = &vertical[y..];

    if substring.starts_with("XMAS") {
        count += 1;
    }

    if substring.starts_with("SAMX") {
        count += 1;
    }

    count
}

fn find_diagonal_forward(lines: &Vec<&str>, x: usize, y: usize, pattern: &str) -> usize {
    let mut count = 0;
    let mut diagonal = String::new();

    let mut dy = y;
    for dx in x..lines[y].len() {
        if dy >= lines.len() {
            break;
        }

        diagonal.push(lines[dy].chars().nth(dx).unwrap());
        dy += 1;
    }

    if diagonal.starts_with(pattern) {
        count += 1;
    }

    let reversed = pattern.chars().rev().collect::<String>();
    if diagonal.starts_with(reversed.as_str()) {
        count += 1;
    }

    count
}

fn find_diagonal_backward(lines: &Vec<&str>, x: usize, y: usize, pattern: &str) -> usize {
    let mut count = 0;
    let mut diagonal = String::new();

    let mut dy = y;
    for dx in (0..x + 1).rev() {
        if dy >= lines.len() {
            break;
        }

        diagonal.push(lines[dy].chars().nth(dx).unwrap());
        dy += 1;
    }

    if diagonal.starts_with(pattern) {
        count += 1;
    }

    let reversed = pattern.chars().rev().collect::<String>();
    if diagonal.starts_with(reversed.as_str()) {
        count += 1;
    }

    count
}

fn find_diagonal(lines: &Vec<&str>, x: usize, y: usize, pattern: &str) -> usize {
    let mut count = 0;

    count += find_diagonal_forward(lines, x, y, pattern);
    count += find_diagonal_backward(lines, x, y, pattern);

    count
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let lines = parse_input(input);

        // find xmas in each direction
        let mut count = 0;

        for (y, line) in lines.iter().enumerate() {
            for (x, _) in line.chars().enumerate() {
                count += find_horizontal(&lines, x, y);
                count += find_vertical(&lines, x, y);
                count += find_diagonal(&lines, x, y, "XMAS");
            }
        }

        Ok(count)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    /**
     * Find the number of times the pattern MAS appears diagonally as an X in the input.
     */
    fn solve(&self, input: &str) -> Result<usize> {
        let lines = parse_input(input);

        // find mas in each direction
        let mut count = 0;

        for (y, line) in lines.iter().enumerate() {
            for (x, _) in line.chars().enumerate() {
                if x + 2 >= lines[y].len() || y + 2 >= lines.len() {
                    continue;
                }

                if find_diagonal_forward(&lines, x, y, "MAS") > 0
                    && find_diagonal_backward(&lines, x + 2, y, "MAS") > 0
                {
                    count += 1;
                }
            }
        }

        Ok(count)
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

    const SAMPLE: &str = r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn part1_sample() {
        let part1 = Part1;

        assert_eq!(part1.solve(SAMPLE).unwrap(), 18);
    }

    #[test]
    fn part2_sample() {
        let part2 = Part2;

        assert_eq!(part2.solve(SAMPLE).unwrap(), 9);
    }
}
