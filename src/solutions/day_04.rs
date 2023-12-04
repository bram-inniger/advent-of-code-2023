use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

pub fn solve_1(cards: Vec<&str>) -> u32 {
    let re_card =
        Regex::new(r"^Card\s+(?<card_id>\d+):(?<winning>(?:\s+\d+)+) \|(?<having>(?:\s+\d+)+)$")
            .unwrap();
    let re_numbers = Regex::new(r"\d+").unwrap();

    parse_cards(cards, &re_card, &re_numbers)
        .iter()
        .map(|c| c.score())
        .sum()
}

fn parse_cards(cards: Vec<&str>, re_card: &Regex, re_numbers: &Regex) -> Vec<Card> {
    cards
        .iter()
        .map(|card| parse_card(card, re_card, re_numbers))
        .collect()
}

fn parse_card(card: &str, re_card: &Regex, re_numbers: &Regex) -> Card {
    let Some(caps) = re_card.captures(card) else {
        panic!("Could not parse: {}", card);
    };

    let _card_id = u32::from_str(&caps["card_id"]).unwrap();
    let winning: HashSet<u32> = re_numbers
        .find_iter(&caps["winning"])
        .map(|m| m.as_str())
        .map(u32::from_str)
        .map(|r| r.unwrap())
        .collect();
    let having: Vec<u32> = re_numbers
        .find_iter(&caps["having"])
        .map(|m| m.as_str())
        .map(u32::from_str)
        .map(|r| r.unwrap())
        .collect();

    Card {
        _card_id,
        winning,
        having,
    }
}

struct Card {
    _card_id: u32,
    winning: HashSet<u32>,
    having: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let matches = self
            .having
            .iter()
            .filter(|n| self.winning.contains(n))
            .count() as u32;

        if matches > 0 {
            2_u32.pow(matches - 1)
        } else {
            0
        }
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
}