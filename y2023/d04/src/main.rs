use std::collections::HashMap;
use aoc::error::{Result};

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let cards = input_to_cards(input)?;

        let mut sum = 0;

        for card in &cards {
            sum += card.score();
        }

        Ok(sum)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let cards = input_to_cards(input)?;

        count_processed_cards(&cards)
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

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
}

impl Card {
    fn from_str(input: &str) -> Result<Self> {
        match regex::Regex::new(r"Card\s+(\d+):(.*)\|(.*)")?.captures(input) {
            Some(captures) => {
                let id = match captures.get(1) {
                    Some(id) => id.as_str().parse::<usize>()?,
                    None => return Err(aoc::error::AocError::ParseError("Failed to parse card id".to_string())),
                };

                let numbers = captures.get(2).ok_or(aoc::error::AocError::ParseError("Failed to parse card numbers".to_string()))?
                    .as_str().trim()
                    .split_whitespace()
                    .map(|num| num.trim().parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                let winning_numbers = captures.get(3).ok_or(aoc::error::AocError::ParseError("Failed to parse card winning numbers".to_string()))?
                    .as_str().trim()
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                Ok(Card {
                    id,
                    numbers,
                    winning_numbers,
                })
            }
            None => Err(aoc::error::AocError::ParseError("Failed to parse card".to_string())),
        }
    }

    fn count_winning_numbers(&self) -> usize {
        let mut count = 0;
        for num in &self.numbers {
            if self.winning_numbers.contains(num) {
                count += 1;
            }
        }
        count
    }

    fn score(&self) -> usize {
        let winning_numbers = self.count_winning_numbers();

        if winning_numbers == 0 {
            return 0;
        }

        // first number is worth 1 point, second doubles the points, etc.
        let mut score = 1;
        for _ in 1..winning_numbers {
            score *= 2;
        }

        score
    }
}

fn input_to_cards(input: &str) -> Result<Vec<Card>> {
    let lines = aoc::split_input(input);

    let mut cards = Vec::new();

    for line in lines {
        let card = Card::from_str(line)?;
        cards.push(card);
    }

    Ok(cards)
}

fn count_processed_cards(cards: &Vec<Card>) -> Result<usize> {
    let mut cards_by_id = HashMap::<usize, Card>::new();
    let mut memory = HashMap::<usize, usize>::new();

    for card in cards {
        cards_by_id.insert(card.id, card.clone());
    }

    fn process_card_by_id(id: usize, memory: &mut HashMap<usize, usize>, cards: &HashMap<usize, Card>) -> Result<usize> {
        // early return if already processed
        if memory.contains_key(&id) {
            return Ok(*memory.get(&id).unwrap());
        }

        let n_winning_numbers = match cards.get(&id) {
            Some(card) => card.count_winning_numbers(),
            None => return Err(aoc::error::AocError::ParseError(format!("Failed to find card with id: {}", id))),
        };

        // early return if no winning numbers
        if n_winning_numbers == 0 {
            memory.insert(id, 1);
            return Ok(1);
        }

        let min_card_id = id + 1;
        let max_card_id = id + n_winning_numbers;

        let mut winning_cards = Vec::new();

        // find all cards with ids between min and max
        for id in min_card_id..=max_card_id {
            let winning_card = match cards.get(&id) {
                Some(card) => card,
                None => return Err(aoc::error::AocError::ParseError(format!("Failed to find card with id: {}", id))),
            };

            winning_cards.push(winning_card.clone());
        }

        let mut n_processed_cards = 1;

        // process all winning cards
        for card in winning_cards {
            let n_processed = process_card_by_id(card.id, memory, cards)?;
            n_processed_cards += n_processed;
        }

        memory.insert(id, n_processed_cards);

        Ok(n_processed_cards)
    }

    let mut n_processed_cards = 0;
    let mut queue = Vec::from(cards.clone());

    // process all cards
    while !queue.is_empty() {
        let card = queue.remove(0);
        let n_processed = process_card_by_id(card.id, &mut memory, &cards_by_id)?;
        n_processed_cards += n_processed;
    }

    Ok(n_processed_cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let lines = aoc::split_input(input);

        let mut cards = Vec::new();

        for line in lines {
            let card = Card::from_str(line).unwrap();
            cards.push(card);
        }

        let mut scores = Vec::new();

        for card in &cards {
            scores.push(card.score());
        }

        assert_eq!(cards.len(), 6);
        assert_eq!(scores, vec![8, 2, 2, 1, 0, 0]);
    }

    #[test]
    fn test_longer_sample() {
        let input = "Card   9: 43 45  1  3 29 75 98 61 11 37 | 59  4 82 11 94 37 85 61 42 49 89 65  6 57 93 51 98 35 78 13  1 32 88 77  8";
        let lines = aoc::split_input(input);
        let card = Card::from_str(lines[0]).unwrap();

        assert_eq!(card.numbers.len(), 10);
        assert_eq!(card.winning_numbers.len(), 25);
    }

    #[test]
    fn test_sample_part2() {
        aoc::init_logging();

        let input = r"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";

        let cards = input_to_cards(input).unwrap();

        let result = count_processed_cards(&cards).unwrap();

        assert_eq!(result, 30);
    }
}