use crate::{day, utils};

pub struct Day03 {}

fn get_bitstrings() -> Vec<String> {
    let input = utils::input(3, false);
    return input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();
}

fn get_rating(use_least_common: bool) -> Option<String> {
    let bitstrings = get_bitstrings();
    let str_length = bitstrings[0].chars().count();
    let mut filtered = bitstrings.clone();

    for i in 0..str_length {
        let mut common_bit = '0';
        let mut count = 0;

        for str in &filtered {
            if str.chars().nth(i).unwrap() == '1' {
                count += 1;
            }
        }

        if count as f32 / filtered.len() as f32 >= 0.5 {
            common_bit = '1';
        }

        if use_least_common {
            common_bit = if common_bit == '1' { '0' } else { '1' };
        }

        filtered = filtered
            .iter()
            .filter(|x| x.chars().nth(i).unwrap() == common_bit)
            .map(|x| x.to_string())
            .collect();

        if filtered.len() == 1 {
            return Some(filtered[0].clone());
        }
    }

    return None;
}

impl day::Day for Day03 {
    fn solve_part1() -> String {
        let bitstrings = get_bitstrings();
        let mut counted_ones = vec![0; bitstrings[0].chars().count()];

        for bitstring in &bitstrings {
            for (i, c) in bitstring.chars().enumerate() {
                if c == '1' {
                    counted_ones[i] += 1;
                }
            }
        }

        let result: String = counted_ones
            .iter()
            .map(|x| {
                if x > &(bitstrings.len() / 2) {
                    '1'
                } else {
                    '0'
                }
            })
            .collect();

        let result_i32 = i32::from_str_radix(&result, 2).unwrap();

        let result_flipped: String = result
            .chars()
            .map(|x| if x == '1' { '0' } else { '1' })
            .collect();

        let result_flipped_i32 = i32::from_str_radix(&result_flipped, 2).unwrap();

        return (result_i32 * result_flipped_i32).to_string();
    }

    fn solve_part2() -> String {
        if let Some((x, y)) = get_rating(false).zip(get_rating(true)) {
            return (isize::from_str_radix(&x, 2).unwrap() * isize::from_str_radix(&y, 2).unwrap())
                .to_string();
        }
        return "".to_string();
    }
}
