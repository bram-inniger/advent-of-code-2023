use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

const BASE_10: u32 = 10;

pub fn solve_1(hands: Vec<&str>) -> u32 {
    Game::new(hands, false).score()
}

pub fn solve_2(hands: Vec<&str>) -> u32 {
    Game::new(hands, true).score()
}

#[derive(Debug)]
struct Game<'a> {
    hands: Vec<Hand<'a>>,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand<'a> {
    h_type: Type<'a>,
    cards: Vec<Rank>,
    bid: u32,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Type<'a> {
    value: u32,
    _name: &'a str,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Rank {
    value: u32,
    _name: char,
}

impl<'a> Game<'a> {
    fn new(hands: Vec<&str>, joker: bool) -> Game {
        Game {
            hands: hands.iter().map(|h| Hand::new(h, joker)).collect(),
        }
    }

    fn score(&self) -> u32 {
        self.hands
            .iter()
            .sorted()
            .map(|h| h.bid)
            .enumerate()
            .map(|(rank, bid)| (rank as u32 + 1) * bid)
            .sum()
    }
}

impl<'a> Hand<'a> {
    fn new(line: &str, joker: bool) -> Hand {
        let line: Vec<&str> = line.split(' ').collect();

        Hand {
            h_type: Type::new(line[0], joker),
            cards: line[0].chars().map(|r| Rank::new(r, joker)).collect(),
            bid: u32::from_str(line[1]).unwrap(),
        }
    }
}

impl<'a> Type<'a> {
    fn new(hand: &'a str, joker: bool) -> Type {
        let hand = Self::resolve_jokers(hand, joker);
        let counts = Self::card_counts(&hand);

        match counts.len() {
            1 => Self::_new(6, "Five of a kind"),
            2 => {
                if counts.values().any(|&c| c == 4) {
                    Self::_new(5, "Four of a kind")
                } else {
                    Self::_new(4, "Full house")
                }
            }
            3 => {
                if counts.values().any(|&c| c == 3) {
                    Self::_new(3, "Three of a kind")
                } else {
                    Self::_new(2, "Two pair")
                }
            }
            4 => Self::_new(1, "One Pair"),
            5 => Self::_new(0, "High card"),
            _ => panic!("Invalid hand: {hand}"),
        }
    }

    fn resolve_jokers(hand: &'a str, joker: bool) -> String {
        let hand_without_jokers = hand.replace('J', "");
        let most_common_card = Self::card_counts(&hand_without_jokers)
            .iter()
            .max_by_key(|e| e.1)
            .map(|e| *e.0)
            .unwrap_or('A');

        if joker {
            hand.replace('J', most_common_card.to_string().as_str())
        } else {
            hand.to_string()
        }
    }

    fn card_counts(hand: &str) -> HashMap<char, u32> {
        hand.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
    }

    // This keeps the auto formatter happy without having to disable it for the block above
    fn _new(value: u32, _name: &'a str) -> Type {
        Type { value, _name }
    }
}

impl Rank {
    fn new(rank: char, joker: bool) -> Rank {
        match rank {
            'A' => Self::_new(14, rank),
            'K' => Self::_new(13, rank),
            'Q' => Self::_new(12, rank),
            'J' => Self::_new(if joker { 1 } else { 11 }, rank),
            'T' => Self::_new(10, rank),
            '2'..='9' => Self::_new(rank.to_digit(BASE_10).unwrap(), rank),
            _ => panic!("Unsupported rank, cannot parse: {rank}"),
        }
    }

    // This keeps the auto formatter happy without having to disable it for the block above
    fn _new(value: u32, _name: char) -> Rank {
        Rank { value, _name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_07_part_01_sample() {
        let sample = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];

        assert_eq!(6_440, solve_1(sample));
    }

    #[test]
    fn day_07_part_01_solution() {
        let input = include_str!("../../inputs/day_07.txt").lines().collect();

        assert_eq!(249_638_405, solve_1(input));
    }

    #[test]
    fn day_07_part_02_sample() {
        let sample = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];

        assert_eq!(5_905, solve_2(sample));
    }

    #[test]
    fn day_07_part_02_solution() {
        let input = include_str!("../../inputs/day_07.txt").lines().collect();

        assert_eq!(249_776_650, solve_2(input));
    }
}
