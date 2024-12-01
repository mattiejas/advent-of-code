use aoc::error::Result;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let numbers: Vec<_> = input.split_whitespace().collect();

        let mut a = Vec::new();
        let mut b = Vec::new();

        for (i, n_str) in numbers.iter().enumerate() {
            let n = n_str.parse::<i32>()?;

            if i % 2 == 0 {
                a.push(n);
            } else {
                b.push(n);
            }
        }

        a.sort_unstable();
        b.sort_unstable();

        let mut sum: i32 = 0;
        for i in 0..a.len() {
            sum += (a[i] - b[i]).abs();
        }

        Ok(sum.try_into().unwrap())
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let numbers: Vec<_> = input.split_whitespace().collect();

        let mut a = Vec::new();
        let mut b = Vec::new();

        for (i, n_str) in numbers.iter().enumerate() {
            let n = n_str.parse::<i32>()?;

            if i % 2 == 0 {
                a.push(n);
            } else {
                b.push(n);
            }
        }

        a.sort_unstable();
        b.sort_unstable();

        let mut sum: i32 = 0;
        for i in 0..a.len() {
            let count = b.iter().filter(|&x| *x == a[i]).count() as i32;
            sum += a[i] * count;
        }

        Ok(sum.try_into().unwrap())
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
