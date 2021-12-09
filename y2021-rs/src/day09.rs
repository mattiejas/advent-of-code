use crate::{day, utils};

pub struct Day09 {}

fn get_heightmap() -> Vec<Vec<i32>> {
    let input = utils::input(9, false);
    return input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.trim()
                .chars()
                .map(|y| y.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
}

// check if point is local minima
fn is_local_minima(heightmap: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let mut local_minima = true;

    match (x, y) {
        (0, 0) => {
            local_minima =
                heightmap[y][x] < heightmap[y][x + 1] && heightmap[y][x] < heightmap[y + 1][x];
        }
        (0, _) => {
            if y == heightmap.len() - 1 {
                local_minima =
                    heightmap[y][x] < heightmap[y][x + 1] && heightmap[y][x] < heightmap[y - 1][x];
            } else {
                local_minima = heightmap[y][x] < heightmap[y][x + 1]
                    && heightmap[y][x] < heightmap[y + 1][x]
                    && heightmap[y][x] < heightmap[y - 1][x];
            }
        }
        (_, 0) => {
            if x == heightmap[y].len() - 1 {
                local_minima =
                    heightmap[y][x] < heightmap[y + 1][x] && heightmap[y][x] < heightmap[y][x - 1];
            } else {
                local_minima = heightmap[y][x] < heightmap[y][x - 1]
                    && heightmap[y][x] < heightmap[y][x + 1]
                    && heightmap[y][x] < heightmap[y + 1][x];
            }
        }
        (_, _) => {
            if x == heightmap[y].len() - 1 && y == heightmap.len() - 1 {
                local_minima =
                    heightmap[y][x] < heightmap[y - 1][x] && heightmap[y][x] < heightmap[y][x - 1];
            } else if x == heightmap.len() - 1 {
                local_minima = heightmap[y][x] < heightmap[y - 1][x]
                    && heightmap[y][x] < heightmap[y + 1][x]
                    && heightmap[y][x] < heightmap[y][x - 1];
            } else if y == heightmap.len() - 1 {
                local_minima = heightmap[y][x] < heightmap[y - 1][x]
                    && heightmap[y][x] < heightmap[y][x + 1]
                    && heightmap[y][x] < heightmap[y][x - 1];
            } else {
                local_minima = heightmap[y][x] < heightmap[y - 1][x]
                    && heightmap[y][x] < heightmap[y + 1][x]
                    && heightmap[y][x] < heightmap[y][x - 1]
                    && heightmap[y][x] < heightmap[y][x + 1];
            }
        }
    }

    return local_minima;
}

impl day::Day for Day09 {
    fn solve_part1() -> String {
        let heightmap = get_heightmap();
        let mut risk_factors = vec![];
        for y in 0..heightmap.len() {
            for x in 0..heightmap[y].len() {
                if is_local_minima(&heightmap, x, y) {
                    risk_factors.push(heightmap[y][x] + 1);
                }
            }
        }
        return risk_factors.iter().sum::<i32>().to_string();
    }

    fn solve_part2() -> String {
        todo!()
    }
}
