use itertools::Itertools;
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

#[rustfmt::skip]
impl<'a> Type<'a> {
    fn new(hand: &'a str, joker: bool) -> Type {
        let hand = Self::resolve_jokers(hand, joker);
        let counts = hand.chars().counts();

        match counts.len() {
            1 => Type { value: 6, _name: "Five of a kind" },
            2 => {
                if counts.values().any(|&c| c == 4) {
                    Type { value: 5, _name: "Four of a kind" }
                } else {
                    Type { value: 4, _name: "Full house" }
                }
            },
            3 => {
                if counts.values().any(|&c| c == 3) {
                    Type { value: 3, _name: "Three of a kind" }
                } else {
                    Type { value: 2, _name: "Two pair" }
                }
            },
            4 => Type { value: 1, _name: "One Pair" },
            5 => Type { value: 0, _name: "High card" },
            _ => panic!("Invalid hand: {hand}"),
        }
    }

    fn resolve_jokers(hand: &'a str, joker: bool) -> String {
        let hand_without_jokers = hand.replace('J', "");
        let most_common_card = hand_without_jokers
            .chars()
            .counts()
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
}

impl Rank {
    fn new(_name: char, joker: bool) -> Rank {
        let value = match _name {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => match joker {
                true => 1,
                false => 11,
            },
            'T' => 10,
            '2'..='9' => _name.to_digit(BASE_10).unwrap(),
            _ => panic!("Unsupported rank, cannot parse: {_name}"),
        };
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
