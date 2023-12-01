use aoc::{Part1, Part2};

struct Solution;

const WRITTEN_NUMBERS: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

impl aoc::Part1<&str, usize> for Solution {
    fn solve_part_one(&self, input: &str) -> aoc::Result<usize> {
        let split_lines = aoc::split_input(input);

        let summed_calibration_value = match sum_calibration_values(&split_lines, false) {
            Ok(value) => value,
            Err(value) => return Err(value),
        };

        Ok(summed_calibration_value)
    }
}

impl aoc::Part2<&str, usize> for Solution {
    fn solve_part_two(&self, input: &str) -> aoc::Result<usize> {
        let split_lines = aoc::split_input(input);

        let summed_calibration_value = match sum_calibration_values(&split_lines, true) {
            Ok(value) => value,
            Err(value) => return Err(value),
        };

        Ok(summed_calibration_value)
    }
}

fn sum_calibration_values(split_lines: &Vec<&str>, expand_written_num: bool) -> aoc::Result<usize> {
    let mut summed_calibration_value = 0;
    for line in split_lines {
        let calibration_value = find_calibration_value(line, expand_written_num);

        let val = match calibration_value {
            Ok(value) => value,
            Err(err) => {
                return Err(aoc::AocError::from_with_msg(
                    err,
                    format!("Failed to find calibration value for line: {}", line).as_str(),
                ))
            }
        };

        summed_calibration_value += val;
    }
    Ok(summed_calibration_value)
}

fn main() {
    let solution = Solution;
    let input = include_str!("../input.txt");

    match solution.solve_part_one(input) {
        Ok(result) => println!("Part one result: {}", result),
        Err(err) => println!("Part one error: {}", err),
    }

    match solution.solve_part_two(input) {
        Ok(result) => println!("Part two result: {}", result),
        Err(err) => println!("Part two error: {}", err),
    }
}

fn find_calibration_value(input: &str, expand_written_num: bool) -> aoc::Result<usize> {
    let mut processed_input = input.to_string();

    if expand_written_num {
        processed_input = expand_written_numbers(input);
    }

    let re = match regex::Regex::new(r"\d") {
        Ok(re) => re,
        Err(err) => return Err(aoc::AocError::from(err)),
    };

    let mut digits: Vec<usize> = Vec::new();

    for capture in re.captures_iter(&processed_input) {
        let digit = capture[0].parse::<usize>().unwrap();
        digits.push(digit);
    }

    if digits.len() == 0 {
        return Err(aoc::AocError::from("No digits found in input"));
    }

    let first_digit = digits[0];
    let last_digit = digits[digits.len() - 1];

    let calibration_value = std::fmt::format(format_args!("{}{}", first_digit, last_digit));

    let parsed_calibration_value = match calibration_value.parse::<usize>() {
        Ok(value) => value,
        Err(err) => return Err(aoc::AocError::from(err)),
    };

    Ok(parsed_calibration_value)
}

fn expand_written_numbers(input: &str) -> String {
    let mut replaced_input = input.to_string();

    for (index, written_number) in WRITTEN_NUMBERS.iter().enumerate() {
        replaced_input = replaced_input.replace(
            written_number,
            format!("{}{}{}", written_number, index, written_number).as_str(),
        );
    }

    replaced_input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_calibration_value_with_one_digit() {
        let input = "9eightone";
        let result = find_calibration_value(input, false);

        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 99);
    }

    #[test]
    fn find_calibration_value_with_two_digits() {
        let input = "9eightone2";
        let result = find_calibration_value(input, false);

        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 92);
    }

    #[test]
    fn find_calibration_value_with_subsequent_digits() {
        let input = "123test";
        let result = find_calibration_value(input, false);

        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 13);
    }

    #[test]
    fn find_calibration_value_replacing_str_with_num() {
        let input = "nineeightone";
        let result = find_calibration_value(input, true);

        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 91);
    }

    #[test]
    fn find_calibration_value_with_overlapping_written_numbers() {
        let input = "nineightwoneight";
        let result = find_calibration_value(input, true);

        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 98);
    }
}
