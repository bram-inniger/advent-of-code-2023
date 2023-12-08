use regex::Regex;
use std::collections::HashMap;

pub fn solve_1(map: Vec<&str>) -> u32 {
    let instructions: Vec<Direction> = map[0].chars().map(Direction::new).collect();

    let nodes = parse_nodes(map);
    let nodes: HashMap<&str, (&str, &str)> = nodes
        .iter()
        .map(|(s, (l, r))| (s.as_str(), (l.as_str(), r.as_str())))
        .collect();

    let mut nr_steps = 0;
    let mut current = "AAA";

    while current != "ZZZ" {
        let direction = &instructions[nr_steps % instructions.len()];
        current = match direction {
            Direction::Left => &nodes[current].0,
            Direction::Right => &nodes[current].1,
        };
        nr_steps += 1;
    }

    nr_steps as u32
}

fn parse_nodes(map: Vec<&str>) -> HashMap<String, (String, String)> {
    let re = Regex::new(r"^(?<start>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)$").unwrap();
    let nodes: HashMap<String, (String, String)> = map.into_iter().skip(2)
        .map(|n| {
            let result = re.captures(n).unwrap();
            let start = result["start"].to_string();
            let left = result["left"].to_string();
            let right = result["right"].to_string();

            (start, (left, right))
        })
        .collect();
    nodes
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(direction: char) -> Direction {
        match direction {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction: {direction}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_08_part_01_sample() {
        let sample = vec![
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ];

        assert_eq!(2, solve_1(sample));

        let sample = vec![
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ];

        assert_eq!(6, solve_1(sample));
    }

    #[test]
    fn day_08_part_01_solution() {
        let input = include_str!("../../inputs/day_08.txt").lines().collect();

        assert_eq!(17_873, solve_1(input));
    }
}
