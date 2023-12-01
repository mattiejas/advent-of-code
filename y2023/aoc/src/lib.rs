use std::fmt;

pub type Result<T> = std::result::Result<T, AocError>;

pub struct Solution {
    input: String,
}

impl Solution {
    pub fn new(input: String) -> Self {
        Solution { input }
    }

    pub fn run<'a, TOutput, TPart>(&'a self, part: &TPart)
    where
        TPart: Part<&'a str, TOutput>,
        TPart: fmt::Debug,
        TOutput: fmt::Display,
    {
        let start = std::time::Instant::now();

        match part.solve(self.input.as_str()) {
            Ok(result) => println!("{:?} result: {}", part, result),
            Err(err) => println!("{:?} error: {}", part, err),
        }
        let duration = start.elapsed();

        println!("Time elapsed solving {:?} is: {:?}\n", part, duration);
    }
}

pub trait Part<TInput, TOutput> {
    fn solve(&self, input: TInput) -> Result<TOutput>;
}

#[derive(Debug, Clone)]
pub struct AocError {
    message: String,
}

impl AocError {
    pub fn from<T: fmt::Display>(error: T) -> Self {
        AocError {
            message: error.to_string(),
        }
    }

    pub fn from_with_msg<T: fmt::Display>(error: T, message: &str) -> Self {
        AocError {
            message: format!("{}: {}", message, error),
        }
    }
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AocError: {}", self.message)
    }
}

pub fn split_input(input: &str) -> Vec<&str> {
    input
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_input_with_newline() {
        let input = "one\ntwo\nthree";
        let result = split_input(input);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "one");
        assert_eq!(result[1], "two");
        assert_eq!(result[2], "three");
    }

    #[test]
    fn split_input_with_carriage_return() {
        let input = "one\r\ntwo\r\nthree";
        let result = split_input(input);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "one");
        assert_eq!(result[1], "two");
        assert_eq!(result[2], "three");
    }

    #[test]
    fn split_input_with_eof() {
        let input = "one\ntwo\nthree\n";
        let result = split_input(input);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "one");
        assert_eq!(result[1], "two");
        assert_eq!(result[2], "three");
    }
}
