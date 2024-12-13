use aoc::error::{AocError, Result};

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

#[derive(Debug, Clone, Copy)]
struct Game {
    button_a: (i64, i64),
    button_b: (i64, i64),

    prize: (i64, i64),
}

fn parse_input(input: &str) -> Vec<Game> {
    let re = regex::Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    re.captures_iter(input.trim())
        .map(|cap| {
            let button_a = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
            let button_b = (cap[3].parse().unwrap(), cap[4].parse().unwrap());
            let prize = (cap[5].parse().unwrap(), cap[6].parse().unwrap());

            Game {
                button_a,
                button_b,
                prize,
            }
        })
        .collect()
}

// cost = 3 * fa + 1 * fb
// x = fa * ax + fb * bx
// y = fa * ay + fb * by
// fa = (x - bx * fb) / ax
// fb = (x * ay - y * ax) / (ay * bx - ax * by)
fn find_tokens(game: &Game) -> Result<i64> {
    let (ax, ay) = game.button_a;
    let (bx, by) = game.button_b;
    let (x, y) = game.prize;

    if bx * ay - by * ax == 0 {
        return Err(AocError::ComputeError("No solution".to_string()));
    }

    let fb = (x * ay - y * ax) / (ay * bx - ax * by);
    let fa = (x - bx * fb) / ax;

    let is_correct = fa * ax + fb * bx == x && fa * ay + fb * by == y;

    if !is_correct {
        return Err(AocError::ComputeError("Invalid solution".to_string()));
    }

    let cost = 3 * fa + fb;

    if cost < 0 {
        return Err(AocError::ComputeError("Negative cost".to_string()));
    }

    Ok(cost)
}

impl aoc::Part<&str, i64> for Part1 {
    fn solve(&self, input: &str) -> Result<i64> {
        let games = parse_input(input);

        let mut total = 0;

        for game in games {
            let cost = find_tokens(&game);
            println!("{:?} => {:?}", game, cost);

            if cost.is_err() {
                continue;
            }

            total += cost.unwrap();
        }

        Ok(total as i64)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let mut games = parse_input(input);

        let mut total = 0;

        for game in &mut games {
            game.prize = (game.prize.0 + 10000000000000, game.prize.1 + 10000000000000);
            let cost = find_tokens(&game);
            println!("{:?} => {:?}", game, cost);

            if cost.is_err() {
                continue;
            }

            total += cost.unwrap() as usize;
        }

        Ok(total)
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

    const SAMPLE: &str = r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;

    #[test]
    fn part1_sample_test() {
        let part1 = Part1;

        assert_eq!(part1.solve(SAMPLE).unwrap(), 480);
    }

    #[test]
    fn part2_sample_test() {
        assert_eq!(1, 1);
    }
}
