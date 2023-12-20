use std::str::FromStr;

use regex::Regex;
use rustc_hash::FxHashSet;

pub fn solve_1(cards: Vec<&str>) -> u32 {
    parse_cards(&cards).iter().map(|c| c.score()).sum()
}

pub fn solve_2(cards: Vec<&str>) -> u32 {
    let cards = parse_cards(&cards);
    let mut pile = vec![1; cards.len() + 1];
    pile[0] = 0; // Sneaky way to 1-index the array

    for card in cards {
        let nr_matches = card.nr_matches();
        let nr_copies = pile[card.card_id as usize];

        for card_id in (&card.card_id + 1)..(&card.card_id + 1 + nr_matches) {
            pile[card_id as usize] += nr_copies;
        }
    }

    pile.iter().sum()
}

fn parse_cards(cards: &[&str]) -> Vec<Card> {
    let re_card =
        Regex::new(r"^Card\s+(?<card_id>\d+):(?<winning>(?:\s+\d+)+) \|(?<having>(?:\s+\d+)+)$")
            .unwrap();
    let re_numbers = Regex::new(r"\d+").unwrap();

    cards
        .iter()
        .map(|card| Card::new(card, &re_card, &re_numbers))
        .collect()
}

struct Card {
    card_id: u32,
    winning: FxHashSet<u32>,
    having: Vec<u32>,
}

impl Card {
    fn new(card: &str, re_card: &Regex, re_numbers: &Regex) -> Card {
        let Some(caps) = re_card.captures(card) else {
            panic!("Could not parse: {}", card);
        };

        let card_id = u32::from_str(&caps["card_id"]).unwrap();
        let winning: FxHashSet<_> = re_numbers
            .find_iter(&caps["winning"])
            .map(|m| m.as_str())
            .map(u32::from_str)
            .map(|r| r.unwrap())
            .collect();
        let having: Vec<_> = re_numbers
            .find_iter(&caps["having"])
            .map(|m| m.as_str())
            .map(u32::from_str)
            .map(|r| r.unwrap())
            .collect();

        Card {
            card_id,
            winning,
            having,
        }
    }

    fn score(&self) -> u32 {
        match Self::nr_matches(self) > 0 {
            true => 2_u32.pow(Self::nr_matches(self) - 1),
            false => 0,
        }
    }

    fn nr_matches(&self) -> u32 {
        self.having
            .iter()
            .filter(|n| self.winning.contains(n))
            .count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_04_part_01_sample() {
        let sample = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        assert_eq!(13, solve_1(sample));
    }

    #[test]
    fn day_04_part_01_solution() {
        let input = include_str!("../../inputs/day_04.txt").lines().collect();

        assert_eq!(23_673, solve_1(input));
    }

    #[test]
    fn day_04_part_02_sample() {
        let sample = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        assert_eq!(30, solve_2(sample));
    }

    #[test]
    fn day_04_part_02_solution() {
        let input = include_str!("../../inputs/day_04.txt").lines().collect();

        assert_eq!(12_263_631, solve_2(input));
    }
}
