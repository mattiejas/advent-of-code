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
    let neighbours = get_neighbours(heightmap, x, y);

    for (x_n, y_n) in neighbours {
        if heightmap[y][x] > heightmap[y_n][x_n] {
            return false;
        }
    }

    return true;
}

fn get_local_minima() -> Vec<(usize, usize)> {
    let heightmap = get_heightmap();
    let mut local_minima: Vec<(usize, usize)> = Vec::new();

    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            if is_local_minima(&heightmap, x, y) {
                local_minima.push((x, y));
            }
        }
    }

    return local_minima;
}

fn get_neighbours(heightmap: &Vec<Vec<i32>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    if x < heightmap[y].len() - 1 {
        neighbours.push((x + 1, y));
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if y < heightmap.len() - 1 {
        neighbours.push((x, y + 1));
    }
    return neighbours;
}

fn expand_basin(heightmap: &Vec<Vec<i32>>, basin: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut expanded_basin: Vec<(usize, usize)> = basin.clone();

    for (x, y) in basin.iter().cloned() {
        let neighbours = get_neighbours(&heightmap, x, y);
        for (x_n, y_n) in neighbours.iter().cloned() {
            if heightmap[y_n][x_n] > heightmap[y][x] && heightmap[y_n][x_n] < 9 {
                if basin
                    .iter()
                    .any(|(x_b, y_b)| x_b.clone() == x_n && y_b.clone() == y_n)
                {
                    continue;
                }
                expanded_basin.push((x_n, y_n));
            }
        }
    }

    return expanded_basin;
}

impl day::Day for Day09 {
    fn solve_part1() -> String {
        let heightmap = get_heightmap();
        return get_local_minima()
            .iter()
            .map(|x| heightmap[x.1][x.0] + 1)
            .sum::<i32>()
            .to_string();
    }

    fn solve_part2() -> String {
        let local_minima = get_local_minima();
        let heightmap = get_heightmap();

        // create initial basins
        let mut basins = Vec::new();
        let mut is_basin_expanded = Vec::new();
        for (x, y) in &local_minima {
            basins.push(expand_basin(&heightmap, &vec![(*x, *y)]));
            is_basin_expanded.push(false);
        }

        // expands basins
        while !is_basin_expanded.iter().all(|x| x.clone()) {
            for (i, basin) in basins.iter_mut().enumerate() {
                if is_basin_expanded[i] {
                    continue;
                }

                let previous = basin.clone();
                basin.extend(expand_basin(&heightmap, &basin));
                basin.sort();
                basin.dedup();

                if previous.len() == basin.len() {
                    is_basin_expanded[i] = true;
                }
            }
        }

        let mut basin_sizes = basins.iter().map(|x| x.len()).collect::<Vec<usize>>();
        basin_sizes.sort();
        basin_sizes.reverse();

        let three_largest = basin_sizes
            .iter()
            .cloned()
            .take(3)
            .reduce(|a, b| a * b)
            .unwrap();

        return three_largest.to_string();
    }
}
