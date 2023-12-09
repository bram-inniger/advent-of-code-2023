use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn solve_1(games: Vec<&str>) -> u32 {
    games
        .iter()
        .map(|g| Game::new(g))
        .filter(|g| g.is_possible())
        .map(|g| g.id)
        .sum()
}

pub fn solve_2(games: Vec<&str>) -> u32 {
    games.iter().map(|g| Game::new(g)).map(|g| g.power()).sum()
}

#[derive(Debug)]
struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

impl Game {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    fn new(game: &str) -> Game {
        let id = u32::from_str(
            &Regex::new(r"Game (?<id>\d+)")
                .unwrap()
                .captures(game)
                .unwrap()["id"],
        )
        .unwrap();

        let max_draw: HashMap<_, _> = Regex::new(r"(\d+ (?:red|green|blue))")
            .unwrap()
            .find_iter(game)
            .map(|m| m.as_str())
            .map(|colour_grab| {
                let nr_to_colour: Vec<_> = colour_grab.split(' ').collect();
                (nr_to_colour[1], u32::from_str(nr_to_colour[0]).unwrap())
            })
            .into_group_map()
            .iter()
            .map(|(&k, v)| (k, *v.iter().max().unwrap()))
            .collect();

        Game {
            id,
            max_red: *max_draw.get("red").unwrap_or(&0),
            max_green: *max_draw.get("green").unwrap_or(&0),
            max_blue: *max_draw.get("blue").unwrap_or(&0),
        }
    }

    fn is_possible(&self) -> bool {
        self.max_red <= Self::MAX_RED
            && self.max_green <= Self::MAX_GREEN
            && self.max_blue <= Self::MAX_BLUE
    }

    fn power(&self) -> u32 {
        self.max_red * self.max_green * self.max_blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_02_part_01_sample() {
        let sample = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        assert_eq!(8, solve_1(sample));
    }

    #[test]
    fn day_02_part_01_solution() {
        let input = include_str!("../../inputs/day_02.txt").lines().collect();

        assert_eq!(2_632, solve_1(input));
    }

    #[test]
    fn day_02_part_02_sample() {
        let sample = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        assert_eq!(2_286, solve_2(sample));
    }

    #[test]
    fn day_02_part_02_solution() {
        let input = include_str!("../../inputs/day_02.txt").lines().collect();

        assert_eq!(69_629, solve_2(input));
    }
}
