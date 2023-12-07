use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

pub fn solve_1(hands: Vec<&str>) -> u64 {
    Game::new(hands).score()
}

#[derive(Debug)]
struct Game<'a> {
    hands: Vec<Hand<'a>>,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand<'a> {
    h_type: Type<'a>,
    cards: Vec<Rank>,
    bid: u64,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Type<'a> {
    value: u64,
    _name: &'a str,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Rank {
    value: u64,
    _name: char,
}

impl<'a> Game<'a> {
    fn new(hands: Vec<&str>) -> Game {
        Game {
            hands: hands.iter().map(|h| Hand::new(h)).collect(),
        }
    }

    fn score(&self) -> u64 {
        self.hands
            .iter()
            .sorted()
            .map(|h| h.bid)
            .enumerate()
            .map(|(rank, bid)| (rank as u64 + 1) * bid)
            .sum()
    }
}

impl<'a> Hand<'a> {
    fn new(line: &str) -> Hand {
        let line: Vec<&str> = line.split(' ').collect();

        Hand {
            h_type: Type::new(line[0]),
            cards: line[0].chars().map(Rank::new).collect(),
            bid: u64::from_str(line[1]).unwrap(),
        }
    }
}

#[rustfmt::skip]
impl<'a> Type<'a> {
    fn new(hand: &str) -> Type {
        let counts: Vec<u64> = hand
            .chars()
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            })
            .values()
            .cloned()
            .collect();

        match counts.len() {
            1 => Type { value: 6, _name: "Five of a kind" },
            2 => {
                if counts.iter().any(|&c| c == 4) {
                    Type { value: 5, _name: "Four of a kind" }
                } else {
                    Type { value: 4, _name: "Full house" }
                }
            }
            3 => {
                if counts.iter().any(|&c| c == 3) {
                    Type { value: 3, _name: "Three of a kind" }
                } else {
                    Type { value: 2, _name: "Two pair" }
                }
            }
            4 => Type { value: 1, _name: "One Pair" },
            5 => Type { value: 0, _name: "High card" },
            _ => panic!("Invalid hand: {hand}"),
        }
    }
}

impl Rank {
    #[rustfmt::skip]
    fn new(rank: char) -> Rank {
        match rank {
            'A' => Rank { value: 14, _name: rank },
            'K' => Rank { value: 13, _name: rank },
            'Q' => Rank { value: 12, _name: rank },
            'J' => Rank { value: 11, _name: rank },
            'T' => Rank { value: 10, _name: rank },
            '9' => Rank { value: 9, _name: rank },
            '8' => Rank { value: 8, _name: rank },
            '7' => Rank { value: 7, _name: rank },
            '6' => Rank { value: 6, _name: rank },
            '5' => Rank { value: 5, _name: rank },
            '4' => Rank { value: 4, _name: rank },
            '3' => Rank { value: 3, _name: rank },
            '2' => Rank { value: 2, _name: rank },
            _ => panic!("Unsupported rank, cannot parse: {rank}"),
        }
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

        assert_eq!(6440, solve_1(sample));
    }

    #[test]
    fn day_07_part_01_solution() {
        let input = include_str!("../../inputs/day_07.txt").lines().collect();

        assert_eq!(249_638_405, solve_1(input));
    }
}
