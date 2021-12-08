use crate::{day, utils};

pub struct Day04 {}

type Card = Vec<Vec<i32>>;

fn get_numbers() -> (Vec<i32>, Vec<Card>) {
    let input = utils::input(4, false);
    let lines: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();

    let numbers: Vec<i32> = lines[0]
        .split(',')
        .map(|s| s.to_string().parse::<i32>())
        .flatten()
        .collect();

    let cards: Vec<Card> = lines
        .iter()
        .skip(1)
        .map(|s| {
            s.split('\n')
                .filter(|s| !s.is_empty())
                .map(|line| {
                    line.split_whitespace()
                        .map(|s| s.to_string().parse::<i32>())
                        .flatten()
                        .collect()
                })
                .collect()
        })
        .collect();

    return (numbers, cards);
}

fn check_rows_for_bingo(card: &Card, numbers: &Vec<i32>) -> bool {
    for row in card {
        let mut found = 0;
        for x in row {
            if !numbers.contains(x) {
                break;
            }
            found += 1;
        }
        if found == 5 {
            return true;
        }
    }
    return false;
}

fn check_columns_for_bingo(card: &Card, numbers: &Vec<i32>) -> bool {
    for col in 0..5 {
        let mut found = 0;
        for row in card {
            if !numbers.contains(&row[col]) {
                break;
            }
            found += 1;
        }
        if found == 5 {
            return true;
        }
    }
    return false;
}

fn get_unmarked_numbers(card: &Card, numbers: &Vec<i32>) -> Vec<i32> {
    let card_copy = card.clone();
    let unmarked_card_numbers = card_copy
        .iter()
        .map(|row| row.clone())
        .flatten()
        .filter(|x| !numbers.contains(x))
        .collect();
    return unmarked_card_numbers;
}

fn get_bingos(remaining_cards: &Vec<Card>, drawn_numbers: &Vec<i32>) -> Vec<Card> {
    let mut bingos: Vec<Card> = Vec::new();

    for card in remaining_cards {
        if check_rows_for_bingo(card, drawn_numbers) || check_columns_for_bingo(card, drawn_numbers)
        {
            bingos.push(card.clone());
        }
    }

    return bingos;
}

impl day::Day for Day04 {
    fn solve_part1() -> String {
        let (numbers, cards) = get_numbers();
        let mut drawn_numbers = Vec::new();

        for number in numbers {
            drawn_numbers.push(number);

            for card in &cards {
                if check_rows_for_bingo(card, &drawn_numbers)
                    || check_columns_for_bingo(card, &drawn_numbers)
                {
                    let unmarked_numbers = get_unmarked_numbers(card, &drawn_numbers);
                    return (unmarked_numbers.iter().sum::<i32>() * number).to_string();
                }
            }
        }

        return "".to_string();
    }

    fn solve_part2() -> String {
        let (numbers, cards) = get_numbers();
        let mut drawn_numbers = Vec::new();
        let mut remaining_cards = (&cards).to_owned();

        'outer: for number in numbers {
            drawn_numbers.push(number);

            let bingos = get_bingos(&remaining_cards, &drawn_numbers);

            for bingo in bingos {
                if remaining_cards.len() == 1 {
                    break 'outer;
                };

                remaining_cards.remove(remaining_cards.iter().position(|x| x == &bingo).unwrap());
            }
        }

        if let Some(card) = remaining_cards.last() {
            let unmarked_numbers = get_unmarked_numbers(card, &drawn_numbers);
            return (unmarked_numbers.iter().sum::<i32>() * drawn_numbers.last().unwrap())
                .to_string();
        }

        return "".to_string();
    }
}
