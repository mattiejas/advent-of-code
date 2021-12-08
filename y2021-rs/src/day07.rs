use crate::day;
use crate::utils;

pub struct Day07;

fn get_crabs() -> Vec<i32> {
    return utils::input(7, false)
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}

fn get_positions() -> Vec<i32> {
    let crabs = get_crabs();

    let positions: Vec<i32> = (0..=crabs.iter().max().unwrap().clone())
        .map(|key| crabs.iter().filter(|&x| x == &key).count() as i32)
        .collect();

    return positions;
}

fn get_nth_triangular_number(n: i32) -> i32 {
    return (n * (n + 1)) / 2;
}

impl day::Day for Day07 {
    fn solve_part1() -> String {
        let positions = get_positions();
        let mut min_cost = std::i32::MAX;

        for i in 0..positions.len() {
            let mut cost = 0;

            for j in 0..positions.len() {
                cost += positions[j] * i32::abs(i as i32 - j as i32);
            }

            if cost < min_cost {
                min_cost = cost;
            }
        }

        return min_cost.to_string();
    }

    fn solve_part2() -> String {
        let positions = get_positions();
        let mut min_cost = std::i32::MAX;

        let mut sorted_positions = positions.iter().enumerate().collect::<Vec<_>>();
        sorted_positions.sort_by(|a, b| a.1.cmp(b.1));
        sorted_positions.reverse();

        for (i, _) in sorted_positions {
            let mut cost = 0;

            for j in 0..positions.len() {
                cost += positions[j] * get_nth_triangular_number(i32::abs(i as i32 - j as i32));

                if cost > min_cost {
                    break;
                }
            }

            if cost < min_cost {
                min_cost = cost;
            }
        }

        return min_cost.to_string();
    }
}
