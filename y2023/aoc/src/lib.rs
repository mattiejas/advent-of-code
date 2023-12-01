use std::fmt;
use log::{error, debug, trace, info};

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
        debug!("Solving {:?}", part);

        let start = std::time::Instant::now();

        match part.solve(self.input.as_str()) {
            Ok(result) => info!("{:?} result: {}", part, result),
            Err(err) => error!("{:?} error: {}", part, err),
        }
        let duration = start.elapsed();

        trace!("Time elapsed solving {:?} = {:?}\n", part, duration);
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
        write!(f, "{}", self.message)
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

pub fn init_logging() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Trace)
        .target(env_logger::Target::Stdout)
        .init();
}
