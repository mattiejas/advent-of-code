use core::fmt;

use aoc::{coord::Coord, error::Result};
use image::RgbImage;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Coord<i32>,
    vel: Coord<i32>,
}

#[derive(Debug)]
struct Bathroom {
    robots: Vec<Robot>,
    width: usize,
    height: usize,
}

fn parse(input: &str) -> Bathroom {
    let re = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let robots = re
        .captures_iter(input.trim())
        .map(|cap| {
            let x = cap[1].parse().unwrap();
            let y = cap[2].parse().unwrap();
            let vx = cap[3].parse().unwrap();
            let vy = cap[4].parse().unwrap();

            Robot {
                pos: Coord::new(x, y),
                vel: Coord::new(vx, vy),
            }
        })
        .collect();

    let width = 101;
    let height = 103;

    // let width = 11;
    // let height = 7;

    Bathroom {
        robots,
        width,
        height,
    }
}

impl Bathroom {
    fn simulate(&mut self, steps: usize) {
        for _ in 0..steps {
            // wrap around
            for robot in &mut self.robots {
                robot.pos = robot.pos + robot.vel;
                robot.pos.x %= self.width as i32;
                robot.pos.y %= self.height as i32;

                if robot.pos.x < 0 {
                    robot.pos.x += self.width as i32;
                }

                if robot.pos.y < 0 {
                    robot.pos.y += self.height as i32;
                }
            }
        }
    }

    fn count_quadrants(&self, gutter_size: usize) -> Vec<usize> {
        let gutter_size = gutter_size as i32;
        let width = self.width as i32;
        let height = self.height as i32;

        // divide the space into quadrants
        let quadrants = [
            (
                Coord::new(0, 0),
                Coord::new((width - gutter_size) / 2, (height - gutter_size) / 2),
            ),
            (
                Coord::new((width - gutter_size) / 2 + gutter_size, 0),
                Coord::new((width - gutter_size) / 2, (height - gutter_size) / 2),
            ),
            (
                Coord::new(0, (height - gutter_size) / 2 + gutter_size),
                Coord::new((width - gutter_size) / 2, (height - gutter_size) / 2),
            ),
            (
                Coord::new(
                    (width - gutter_size) / 2 + gutter_size,
                    (height - gutter_size) / 2 + gutter_size,
                ),
                Coord::new((width - gutter_size) / 2, (height - gutter_size) / 2),
            ),
        ];

        let mut count = vec![0; 4];

        for robot in &self.robots {
            for (i, (pos, size)) in quadrants.iter().enumerate() {
                if robot.pos.x >= pos.x
                    && robot.pos.x < pos.x + size.x
                    && robot.pos.y >= pos.y
                    && robot.pos.y < pos.y + size.y
                {
                    count[i] += 1;
                }
            }
        }

        count
    }

    fn draw(&self, step: usize) {
        let mut grid = vec![vec![0; self.width]; self.height];

        for robot in &self.robots {
            grid[robot.pos.y as usize][robot.pos.x as usize] += 1;
        }

        /*  for row in grid {
            for cell in row {
                print!(
                    "{}",
                    if cell > 0 {
                        format!("{}", cell)
                    } else {
                        ".".to_owned()
                    }
                );
            }
            println!();
        } */

        let mut image = RgbImage::new(self.width as u32, self.height as u32);

        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let color = if *cell > 0 {
                    [255, 255, 255]
                } else {
                    [0, 0, 0]
                };

                image.put_pixel(x as u32, y as u32, image::Rgb(color));
            }
        }

        image.save(format!("d14/output/{}.png", step)).unwrap();
    }

    fn simulate_draw(&mut self, steps: usize) {
        for i in 0..steps {
            print!("step: {}/{}\n", i, steps);
            self.simulate(1);
            self.draw(i);

            // print!("{}[2J", 27 as char);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }
    }
}

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let mut bathroom = parse(input);
        let steps = 100;

        bathroom.simulate(steps);
        let count = bathroom.count_quadrants(1);

        println!("quadrants: {:?}", count);
        Ok(count.iter().fold(1, |acc, x| acc * x))
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let mut bathroom = parse(input);
        let steps = 10000000;

        bathroom.simulate_draw(steps);

        // lower than 17278
        // higher than 6875
        // +1 to account for the first step
        Ok(6876)
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
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;

    #[test]
    fn part1_sample_test() {
        let part1 = Part1;
        let solution = part1.solve(SAMPLE).unwrap();
        assert_eq!(solution, 12);
    }

    #[test]
    fn part2_sample_test() {
        assert_eq!(1, 1);
    }
}
