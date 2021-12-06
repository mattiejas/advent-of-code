use crate::day::Day;
use crate::utils;

pub struct Day05 {}

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Copy, Clone)]
struct LineSegment {
    start: Point,
    end: Point,
}

fn get_line_segments() -> Vec<LineSegment> {
    let input = utils::input(5, false);
    return input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|line| -> Vec<Point> {
            return line
                .split(" -> ")
                .map(|s| -> Vec<i32> {
                    return s
                        .trim()
                        .split(',')
                        .map(|str| str.trim().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                })
                .map(|p| {
                    return Point { x: p[0], y: p[1] };
                })
                .collect();
        })
        .map(|p| LineSegment {
            start: p[0],
            end: p[1],
        })
        .collect();
}

fn max_x_y(line_segments: &Vec<LineSegment>) -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;
    for line_segment in line_segments {
        if line_segment.start.x > max_x {
            max_x = line_segment.start.x;
        }
        if line_segment.start.y > max_y {
            max_y = line_segment.start.y;
        }
        if line_segment.end.x > max_x {
            max_x = line_segment.end.x;
        }
        if line_segment.end.y > max_y {
            max_y = line_segment.end.y;
        }
    }
    return (max_x + 1, max_y + 1);
}

impl Day for Day05 {
    fn solve_part1() -> String {
        let points = get_line_segments();

        let (max_x, max_y) = max_x_y(&points);
        let mut map = vec![vec![0; max_y as usize]; max_x as usize];

        for ls in points {
            println!(
                "{}, {} -> {}, {}",
                ls.start.x, ls.start.y, ls.end.x, ls.end.y
            );

            if ls.start.x == ls.end.x || ls.start.y == ls.end.y {
                for x in std::cmp::min(ls.start.x, ls.end.x)..=(std::cmp::max(ls.end.x, ls.start.x))
                {
                    for y in
                        std::cmp::min(ls.start.y, ls.end.y)..=(std::cmp::max(ls.end.y, ls.start.y))
                    {
                        map[x as usize][y as usize] += 1;
                    }
                }
            }
        }

        return map
            .iter()
            .map(|v| v.iter().filter(|&x| *x > 1).count())
            .sum::<usize>()
            .to_string();
    }

    fn solve_part2() -> String {
        let points = get_line_segments();
        let (max_x, max_y) = max_x_y(&points);
        let mut map = vec![vec![0; max_y as usize]; max_x as usize];

        for ls in points {
            println!(
                "{}, {} -> {}, {}",
                ls.start.x, ls.start.y, ls.end.x, ls.end.y
            );

            if ls.start.x == ls.end.x || ls.start.y == ls.end.y {
                for x in std::cmp::min(ls.start.x, ls.end.x)..=(std::cmp::max(ls.end.x, ls.start.x))
                {
                    for y in
                        std::cmp::min(ls.start.y, ls.end.y)..=(std::cmp::max(ls.end.y, ls.start.y))
                    {
                        map[x as usize][y as usize] += 1;
                    }
                }
            } else {
                let y_min = std::cmp::min(ls.start.y, ls.end.y);
                let y_max = std::cmp::max(ls.start.y, ls.end.y);

                let x_sign = if ls.start.x < ls.end.x { 1 } else { -1 };
                let y_sign = if ls.start.y < ls.end.y { 1 } else { -1 };

                let y_diff = y_max - y_min;
                for step in 0..=y_diff {
                    map[(ls.start.x + x_sign * step) as usize]
                        [(ls.start.y + y_sign * step) as usize] += 1;
                }
            }
        }

        return map
            .iter()
            .map(|v| v.iter().filter(|&x| *x > 1).count())
            .sum::<usize>()
            .to_string();
    }
}
