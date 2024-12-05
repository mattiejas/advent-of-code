use aoc::error::{AocError, Result};
use itertools::Itertools;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

#[derive(Debug)]
struct Rule {
    before: usize,
    after: usize,
}

#[derive(Debug, Clone)]
struct Update {
    pages: Vec<usize>,
}

fn parse_input(input: &str) -> Result<(Vec<Rule>, Vec<Update>)> {
    // split on blank line
    let lines = input.trim().lines().collect::<Vec<&str>>();

    let rules = lines
        .iter()
        .filter(|line| line.contains("|"))
        .map(|line| {
            let (before, after) = line.split("|").collect_tuple().unwrap();

            let a = before.parse::<usize>().unwrap();
            let b = after.parse::<usize>().unwrap();

            Rule {
                before: a,
                after: b,
            }
        })
        .collect();

    let updates = lines
        .iter()
        .filter(|line| line.contains(","))
        .map(|line| {
            let pages = line
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            Update { pages }
        })
        .collect();

    Ok((rules, updates))
}

fn is_correct_order(rules: &Vec<Rule>, update: &Update) -> bool {
    for rule in rules {
        // find if the pages are in the update
        let before_index = update.pages.iter().position(|p| *p == rule.before);
        let after_index = update.pages.iter().position(|p| *p == rule.after);

        if let (Some(before_index), Some(after_index)) = (before_index, after_index) {
            if before_index < after_index {
                continue;
            } else {
                return false;
            }
        }
    }

    true
}

fn ensure_order(rules: &Vec<Rule>, update: &Update) -> Update {
    let mut new_update = update.clone();

    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 0..new_update.pages.len() - 1 {
            let a = new_update.pages[i];
            let b = new_update.pages[i + 1];

            for rule in rules {
                if rule.before == a && rule.after == b {
                    continue;
                }

                if rule.before == b && rule.after == a {
                    new_update.pages.swap(i, i + 1);
                    swapped = true;
                    break;
                }
            }
        }
    }

    new_update
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let (rules, updates) = parse_input(input)?;

        let sum: usize = updates
            .iter()
            .filter(|update| is_correct_order(&rules, update))
            .map(|update| update.pages[update.pages.len() / 2])
            .sum();

        Ok(sum)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let (rules, updates) = parse_input(input)?;

        let sum = updates
            .iter()
            .filter(|update| !is_correct_order(&rules, update))
            .map(|update| ensure_order(&rules, update))
            .map(|update| update.pages[update.pages.len() / 2])
            .sum();

        Ok(sum)
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

    const SAMPLE: &str = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn part1_test() {
        let part1 = Part1;
        assert_eq!(part1.solve(&SAMPLE).unwrap(), 143);
    }

    #[test]
    fn part2_test() {
        let part2 = Part2;
        assert_eq!(part2.solve(&SAMPLE).unwrap(), 123);
    }
}
