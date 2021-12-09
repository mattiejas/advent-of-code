use crate::{day, utils};

pub struct Day09 {}

fn get_heightmap() -> Vec<Vec<i32>> {
    let input = utils::input(9, true);
    return input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i32).collect())
        .collect();
}

fn get_gradient(heightmap: &Vec<Vec<i32>>, x: usize, y: usize) -> f32 {
    let height = heightmap[y][x];
    let nx = heightmap[0].len() - 1;
    let ny = heightmap.len() - 1;

    let mut dx = 0.;
    let mut dy = 0.;

    match x {
        0 => {
            dx += (heightmap[y][x + 1] - height) as f32;
        }
        nx => {
            dx += (heightmap[y][x - 1] - height) as f32;
        }
        _ => {
            dx += (heightmap[y][x - 1] - height) as f32;
            dx += (heightmap[y][x + 1] - height) as f32;
            dx /= 2.;
        }
    }

    match y {
        0 => {
            dy += (heightmap[y + 1][x] - height) as f32;
        }
        Some(n) if n == y => {
            dy += (heightmap[y - 1][x] - height) as f32;
        }
        _ => {
            dy += (heightmap[y - 1][x] - height) as f32;
            dy += (heightmap[y + 1][x] - height) as f32;
            dy /= 2.;
        }
    }

    return (dx.powi(2) + dy.powi(2)).sqrt();
}

fn get_gradients(heightmap: &Vec<Vec<i32>>) -> Vec<Vec<f32>> {
    let mut gradients = vec![vec![0.; heightmap[0].len()]; heightmap.len()];

    for y in 0..heightmap.len() {
        for x in 0..heightmap[0].len() {
            gradients[y][x] = get_gradient(heightmap, x, y);
        }
    }

    gradients
}

impl day::Day for Day09 {
    fn solve_part1() -> String {
        let gradients = get_gradients(&get_heightmap());
        println!("{:#?}", gradients);
        return "".to_string();
    }

    fn solve_part2() -> String {
        todo!()
    }
}
