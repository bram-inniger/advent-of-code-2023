use crate::util::lcm;
use std::collections::{HashMap, HashSet};
use std::ops::{Index, Not};

pub fn solve_1(map: Vec<&str>) -> u64 {
    solve(map, "AAA", "ZZZ")
}

pub fn solve_2(map: Vec<&str>) -> u64 {
    solve(map, "A", "Z")
}

pub fn solve(map: Vec<&str>, start_suffix: &str, end_suffix: &str) -> u64 {
    let instruction = Instruction::new(map[0]);
    let graph = Graph::new(&map.iter().skip(2).cloned().collect::<Vec<&str>>());

    graph
        .start_values()
        .iter()
        .filter(|k| k.ends_with(start_suffix))
        .map(|start| find_cycle_length(&instruction, &graph, start, end_suffix))
        .fold(1, lcm)
}

fn find_cycle_length(instr: &Instruction, graph: &Graph, start: &str, end: &str) -> u64 {
    let mut nr_steps = 0;
    let mut current = start;

    while current.ends_with(end).not() {
        current = match instr[nr_steps] {
            Direction::Left => graph.left(current),
            Direction::Right => graph.right(current),
        };
        nr_steps += 1;
    }

    nr_steps
}

#[derive(Debug)]
struct Instruction {
    directions: Vec<Direction>,
}

impl Instruction {
    fn new(instruction: &str) -> Instruction {
        Instruction {
            directions: instruction.chars().map(Direction::new).collect(),
        }
    }
}

impl Index<u64> for Instruction {
    type Output = Direction;

    fn index(&self, index: u64) -> &Self::Output {
        &self.directions[(index as usize) % self.directions.len()]
    }
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

#[derive(Debug)]
struct Graph<'a> {
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Graph<'a> {
    fn new(map: &[&'a str]) -> Graph<'a> {
        let nodes = map
            .iter()
            .map(|&n| {
                let start = &n[0..3];
                let left = &n[7..10];
                let right = &n[12..15];

                (start, (left, right))
            })
            .collect();
        Graph { nodes }
    }

    fn start_values(&self) -> HashSet<&'a str> {
        self.nodes.keys().copied().collect()
    }

    fn left(&self, node: &str) -> &'a str {
        self.nodes[node].0
    }

    fn right(&self, node: &str) -> &'a str {
        self.nodes[node].1
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

    #[test]
    fn day_08_part_02_sample() {
        let sample = vec![
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ];

        assert_eq!(6, solve_2(sample));
    }

    #[test]
    fn day_08_part_02_solution() {
        let input = include_str!("../../inputs/day_08.txt").lines().collect();

        assert_eq!(15_746_133_679_061, solve_2(input));
    }
}
