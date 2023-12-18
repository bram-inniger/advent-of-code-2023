use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;

const BASE_16: u32 = 16;

pub fn solve_1(plan: Vec<&str>) -> u64 {
    Terrain::new(plan, false).dig_lagoon()
}

pub fn solve_2(plan: Vec<&str>) -> u64 {
    Terrain::new(plan, true).dig_lagoon()
}

#[derive(Debug)]
struct Terrain {
    instructions: Vec<Instruction>,
}

impl Terrain {
    fn new(plan: Vec<&str>, swapped: bool) -> Terrain {
        let re = Regex::new(r"^(?<direction>[URDL]) (?<steps>\d+) \(#(?<colour>.+)\)$").unwrap();

        let instructions = plan
            .iter()
            .map(|&hay| {
                let caps = re.captures(hay).unwrap();

                let direction = if swapped {
                    match &caps["colour"][5..] {
                        "0" => Direction::Right,
                        "1" => Direction::Down,
                        "2" => Direction::Left,
                        "3" => Direction::Up,
                        _ => unreachable!(),
                    }
                } else {
                    match &caps["direction"] {
                        "R" => Direction::Right,
                        "D" => Direction::Down,
                        "L" => Direction::Left,
                        "U" => Direction::Up,
                        _ => unreachable!(),
                    }
                };
                let steps = if swapped {
                    u32::from_str_radix(&caps["colour"][..5], BASE_16).unwrap()
                } else {
                    u32::from_str(&caps["steps"]).unwrap()
                };

                Instruction { direction, steps }
            })
            .collect_vec();

        Terrain { instructions }
    }

    fn dig_lagoon(&self) -> u64 {
        let mut current = (0, 0);
        let mut perimeter = 0;
        let mut coords = Vec::new();

        coords.push(current);

        for instruction in &self.instructions {
            match instruction.direction {
                Direction::Up => current.1 -= instruction.steps as i64,
                Direction::Right => current.0 += instruction.steps as i64,
                Direction::Down => current.1 += instruction.steps as i64,
                Direction::Left => current.0 -= instruction.steps as i64,
            }

            perimeter += instruction.steps as u64;
            coords.push(current);
        }

        // Shoelace formula
        let mut triangles_sum = 0;
        for idx in 0..coords.len() - 1 {
            let (x1, y1) = coords[idx];
            let (x2, y2) = coords[idx + 1];

            triangles_sum += (y1 + y2) * (x1 - x2);
        }
        let area = (triangles_sum / 2).unsigned_abs();

        // Pick's theorem
        area + perimeter / 2 + 1
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: u32,
}

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_18_part_01_sample() {
        let sample = vec![
            "R 6 (#70c710)",
            "D 5 (#0dc571)",
            "L 2 (#5713f0)",
            "D 2 (#d2c081)",
            "R 2 (#59c680)",
            "D 2 (#411b91)",
            "L 5 (#8ceee2)",
            "U 2 (#caa173)",
            "L 1 (#1b58a2)",
            "U 2 (#caa171)",
            "R 2 (#7807d2)",
            "U 3 (#a77fa3)",
            "L 2 (#015232)",
            "U 2 (#7a21e3)",
        ];

        assert_eq!(62, solve_1(sample));
    }

    #[test]
    fn day_18_part_01_solution() {
        let input = include_str!("../../inputs/day_18.txt").lines().collect();

        assert_eq!(72_821, solve_1(input));
    }

    #[test]
    fn day_18_part_02_sample() {
        let sample = vec![
            "R 6 (#70c710)",
            "D 5 (#0dc571)",
            "L 2 (#5713f0)",
            "D 2 (#d2c081)",
            "R 2 (#59c680)",
            "D 2 (#411b91)",
            "L 5 (#8ceee2)",
            "U 2 (#caa173)",
            "L 1 (#1b58a2)",
            "U 2 (#caa171)",
            "R 2 (#7807d2)",
            "U 3 (#a77fa3)",
            "L 2 (#015232)",
            "U 2 (#7a21e3)",
        ];

        assert_eq!(952_408_144_115, solve_2(sample));
    }

    #[test]
    fn day_18_part_02_solution() {
        let input = include_str!("../../inputs/day_18.txt").lines().collect();

        assert_eq!(127_844_509_405_501, solve_2(input));
    }
}
