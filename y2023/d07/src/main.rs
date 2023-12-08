use aoc::error::Result;

#[derive(Debug)]
struct Part1;

#[derive(Debug)]
struct Part2;

impl aoc::Part<&str, usize> for Part1 {
    fn solve(&self, input: &str) -> Result<usize> {
        let mut hands = parse_input(input, false).unwrap();

        hands.sort_by(|a, b| a.cmp(b));

        let winnings = hands.iter().enumerate().fold(0, |winnings, (i, hand)| {
            return winnings + hand.bid * (i + 1);
        });

        Ok(winnings)
    }
}

impl aoc::Part<&str, usize> for Part2 {
    fn solve(&self, input: &str) -> Result<usize> {
        let mut hands = parse_input(input, true).unwrap();

        hands.sort_by(|a, b| a.cmp(b));

        let winnings = hands.iter().enumerate().fold(0, |winnings, (i, hand)| {
            return winnings + hand.bid * (i + 1);
        });

        Ok(winnings)
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum SpecialHand {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Card {
    fn from_char(c: char, with_joker: bool) -> Option<Card> {
        match c {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => with_joker.then(|| Card::Joker).or_else(|| Some(Card::Jack)),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
    special_hand: Option<SpecialHand>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_special = self.special_hand;
        let other_special = other.special_hand;

        if self_special.is_some() && other_special.is_none() {
            return std::cmp::Ordering::Greater;
        }

        if self_special.is_none() && other_special.is_some() {
            return std::cmp::Ordering::Less;
        }

        if self_special.is_some() && other_special.is_some() && self_special != other_special {
            return self_special.unwrap().cmp(&other_special.unwrap());
        }

        // if the hands are the same, compare each card
        // highest card wins

        for i in 0..5 {
            let comparison = self.cards[i].cmp(&other.cards[i]);

            if comparison != std::cmp::Ordering::Equal {
                return comparison;
            }
        }

        return std::cmp::Ordering::Equal;
    }
}

impl Hand {
    fn new(cards: [Card; 5], bid: usize) -> Self {
        let mut hand = Hand {
            cards,
            bid,
            special_hand: None,
        };
        let mut counts = [0; 14];
        let mut joker_count = 0;

        for card in cards.iter() {
            if *card == Card::Joker {
                joker_count += 1;
            } else {
                counts[*card as usize] += 1;
            }
        }

        let mut counts = counts.to_vec();
        counts.sort();
        counts.reverse();

        if joker_count > 0 {
            counts[0] += joker_count;
        }

        hand.special_hand = match counts[0..2] {
            [a, _] if a == 5 => Some(SpecialHand::FiveOfAKind),
            [a, _] if a == 4 => Some(SpecialHand::FourOfAKind),
            [a, b] if a == 3 && b == 2 => Some(SpecialHand::FullHouse),
            [a, _] if a == 3 => Some(SpecialHand::ThreeOfAKind),
            [a, b] if a == 2 && b == 2 => Some(SpecialHand::TwoPairs),
            [a, _] if a == 2 => Some(SpecialHand::OnePair),
            _ => Some(SpecialHand::HighCard),
        };

        hand
    }
}

fn parse_input(input: &str, with_joker: bool) -> Result<Vec<Hand>> {
    let lines = aoc::split_input(input);

    Ok(lines
        .iter()
        .map(|line| {
            let mut cards = [Card::Two; 5];

            let (cards_str, bid_str) = line.split_at(5);

            for (i, c) in cards_str.chars().enumerate() {
                cards[i] = Card::from_char(c, with_joker).unwrap();
            }

            let bid = bid_str.trim().parse::<usize>().unwrap();

            Hand::new(cards, bid)
        })
        .collect::<Vec<Hand>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let hands = parse_input(input, false).unwrap();

        assert_eq!(hands.len(), 5);
        assert_eq!(
            hands[0].cards,
            [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]
        );
        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[0].special_hand, Some(SpecialHand::OnePair));

        let hand5 = hands.get(4).unwrap();
        assert_eq!(
            hand5.cards,
            [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace]
        );
        assert_eq!(hand5.bid, 483);
        assert_eq!(hand5.special_hand, Some(SpecialHand::ThreeOfAKind));

        let mut hands = hands;
        hands.sort_by(|a, b| a.cmp(b));
        dbg!(&hands);

        let winnings = hands.iter().enumerate().fold(0, |winnings, (i, hand)| {
            return winnings + hand.bid * (i + 1);
        });

        assert_eq!(winnings, 6440);
    }
}
