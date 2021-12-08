use std::collections::HashMap;

use crate::day;
use crate::utils;

pub struct Day08;

fn get_input() -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let lines: Vec<Vec<String>> = utils::input(8, false)
        .trim()
        .split('\n')
        .map(|x| x.split('|').map(|y| y.to_string()).collect())
        .collect();

    let segments: Vec<Vec<String>> = lines
        .clone()
        .into_iter()
        .map(|x| x.into_iter())
        .map(|mut x| {
            x.next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect()
        })
        .collect();

    let output: Vec<Vec<String>> = lines
        .clone()
        .into_iter()
        .map(|x| x.into_iter())
        .map(|x| {
            x.skip(1)
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect()
        })
        .collect();

    (segments, output)
}

fn intersection(a: &str, b: &str) -> String {
    let mut result = String::new();
    a.chars().into_iter().for_each(|x| {
        if b.contains(x) {
            result.push(x);
        }
    });
    return result;
}

fn diff(set: &str, other: &str) -> String {
    let mut result = String::new();
    set.chars().into_iter().for_each(|x| {
        if !other.contains(x) {
            result.push(x);
        }
    });
    return result;
}

fn union(a: &str, b: &str) -> String {
    let mut result = String::new();
    a.chars().into_iter().for_each(|x| {
        if !result.contains(x) {
            result.push(x);
        }
    });
    b.chars().into_iter().for_each(|x| {
        if !result.contains(x) {
            result.push(x);
        }
    });
    return result;
}

fn sort(a: &str) -> String {
    let mut chars: Vec<char> = a.chars().into_iter().collect();
    chars.sort();
    return chars.into_iter().collect();
}

fn find_configuration(segments: &Vec<String>) -> HashMap<String, u8> {
    let mut filtered_segments = segments.clone();
    let mut config = vec![String::new(); 10];

    // get trivial numbers
    if let Some(x) = segments.iter().find(|x| x.chars().count() == 2) {
        config[1] = x.clone();
        filtered_segments.retain(|y| y != x);
    }
    if let Some(x) = segments.iter().find(|x| x.chars().count() == 4) {
        config[4] = x.clone();
        filtered_segments.retain(|y| y != x);
    }
    if let Some(x) = segments.iter().find(|x| x.chars().count() == 3) {
        config[7] = x.clone();
        filtered_segments.retain(|y| y != x);
    }
    if let Some(x) = segments.iter().find(|x| x.chars().count() == 7) {
        config[8] = x.clone();
        filtered_segments.retain(|y| y != x);
    }

    // figure out the rest
    // 6 = 8 - 7 + upper segment
    for (i, seg) in filtered_segments.iter_mut().enumerate() {
        if seg.chars().count() == 6 {
            if sort(&union(seg, &config[7])) == sort(&config[8]) {
                config[6] = seg.clone();
                filtered_segments.remove(i);
                break;
            }
        }
    }

    // left lower = 6 - 5
    for (i, seg) in filtered_segments.iter_mut().enumerate() {
        if seg.chars().count() == 5 {
            if intersection(seg, &config[6]).chars().count() == 5 {
                config[5] = seg.clone();
                filtered_segments.remove(i);
                break;
            }
        }
    }

    let left_lower_segment: String = diff(&config[6], &config[5]);
    let temp_left_middle_segments: String = diff(&config[4], &config[7]);

    // find 9
    for (i, seg) in filtered_segments.iter_mut().enumerate() {
        if seg.chars().count() == 6 {
            if diff(&config[8], seg) == left_lower_segment {
                config[9] = seg.clone();
                filtered_segments.remove(i);
                break;
            }
        }
    }

    // find 0
    for (i, seg) in filtered_segments.iter_mut().enumerate() {
        if seg.chars().count() == 6 {
            config[0] = seg.clone(); // only remaining number with 6 segments
            filtered_segments.remove(i);
            break;
        }
    }

    let middle_segment = diff(&config[8], &config[0]);
    let left_upper_segment = diff(&temp_left_middle_segments, &middle_segment);

    // find 3
    for (i, seg) in filtered_segments.iter_mut().enumerate() {
        if diff(&config[9], seg) == left_upper_segment {
            config[3] = seg.clone();
            filtered_segments.remove(i);
            break;
        }
    }

    // find 2
    for seg in filtered_segments {
        config[2] = seg.clone();
        break;
    }

    // sort keys and put into hashmap
    let mut result = HashMap::new();
    for (i, x) in config.iter().enumerate() {
        result.insert(sort(x), i as u8);
    }
    return result;
}

impl day::Day for Day08 {
    fn solve_part1() -> String {
        let (_segments, output) = get_input();
        let unique_lengths = vec![2, 3, 4, 7];

        return output
            .iter()
            .map(|x| {
                x.iter()
                    .filter(|y| unique_lengths.contains(&y.len()))
                    .count()
            })
            .sum::<usize>()
            .to_string();
    }

    fn solve_part2() -> String {
        let (segments, output) = get_input();
        let mut results: Vec<u32> = vec![];

        for (s, o) in segments.iter().zip(output.iter()) {
            let config = find_configuration(s);
            let numbers: Vec<u8> = o.iter().map(|x| config[&sort(x)]).collect();

            let mut result = String::new();
            for x in numbers {
                result.push_str(&x.to_string());
            }
            results.push(result.parse::<u32>().unwrap());
        }

        return results.iter().sum::<u32>().to_string();
    }
}
