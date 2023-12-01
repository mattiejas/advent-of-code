use aoc::Part1;

pub struct Solution;

impl aoc::Part1<&str, usize> for Solution {
    fn solve_part_one(&self, input: &str) -> aoc::Result<usize> {
        let split_lines = aoc::split_input(input);
        let mut summed_calibration_value = 0;

        for line in split_lines {
            let calibration_value = find_calibration_value(line);

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
}

fn main() {
    let solution = Solution;
    let input_part_one = include_str!("../input-part-one.txt");

    match solution.solve_part_one(input_part_one) {
        Ok(result) => println!("Part one result: {}", result),
        Err(err) => println!("Part one error: {}", err),
    }
}

fn find_calibration_value(input: &str) -> aoc::Result<usize> {
    let re = match regex::Regex::new(r"\d") {
        Ok(re) => re,
        Err(err) => return Err(aoc::AocError::from(err)),
    };

    let mut digits: Vec<usize> = Vec::new();

    for capture in re.captures_iter(input) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_calibration_value_with_one_digit() {
        let input = "9eightone";
        let result = find_calibration_value(input);

        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 99);
    }

    #[test]
    fn find_calibration_value_with_two_digits() {
        let input = "9eightone2";
        let result = find_calibration_value(input);

        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 92);
    }

    #[test]
    fn find_calibration_value_with_subsequent_digits() {
        let input = "123test";
        let result = find_calibration_value(input);

        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 13);
    }
}
