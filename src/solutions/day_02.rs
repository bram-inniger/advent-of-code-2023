use regex::Regex;
use std::cmp::max;
use std::str::FromStr;

pub fn solve_1(games: Vec<&str>) -> u32 {
    parse_games(games)
        .iter()
        .filter(|g| g.is_possible())
        .map(|g| g.id)
        .sum()
}

pub fn solve_2(games: Vec<&str>) -> u32 {
    parse_games(games)
        .iter()
        .map(|g| g.power())
        .sum()
}

fn parse_games(games: Vec<&str>) -> Vec<Game> {
    let id_re = Regex::new(r"Game (?<id>\d+)").unwrap();
    let draw_re = Regex::new(r"(\d+ (?:red|green|blue))").unwrap();

    let mut game_summaries: Vec<Game> = Vec::new();

    for game in games {
        let id_captures = id_re.captures(game).unwrap();
        let id = u32::from_str(&id_captures["id"]).unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for draw in draw_re.find_iter(game).map(|m| m.as_str()) {
            match draw {
                r if r.ends_with(" red") => {
                    max_red = max(max_red, u32::from_str(r.trim_end_matches(" red")).unwrap())
                }
                g if g.ends_with(" green") => {
                    max_green = max(
                        max_green,
                        u32::from_str(g.trim_end_matches(" green")).unwrap(),
                    )
                }
                b if b.ends_with(" blue") => {
                    max_blue = max(
                        max_blue,
                        u32::from_str(b.trim_end_matches(" blue")).unwrap(),
                    )
                }
                no_match => panic!("Couldn't find any matches in: {}", no_match),
            }
        }

        game_summaries.push(Game {
            id,
            max_red,
            max_green,
            max_blue,
        })
    }

    game_summaries
}

struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.max_red <= 12 && self.max_green <= 13 && self.max_blue <= 14
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

        assert_eq!(8, solve_1(sample))
    }

    #[test]
    fn day_02_part_01_solution() {
        let input = include_str!("../../inputs/day_02.txt").lines().collect();

        assert_eq!(2_632, solve_1(input))
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

        assert_eq!(2_286, solve_2(sample))
    }

    #[test]
    fn day_02_part_02_solution() {
        let input = include_str!("../../inputs/day_02.txt").lines().collect();

        assert_eq!(69_629, solve_2(input))
    }
}
