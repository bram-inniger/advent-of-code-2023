use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::ops::{Index, Not};

const BASE_10: u32 = 10;

type Coord = (i16, i16);

pub fn solve_1(city: Vec<&str>) -> u32 {
    City::new(city).shortest_path(false)
}

pub fn solve_2(city: Vec<&str>) -> u32 {
    City::new(city).shortest_path(true)
}

#[derive(Debug)]
struct City {
    blocks: Vec<Vec<u32>>,
    height: i16,
    width: i16,
}

impl City {
    fn new(city: Vec<&str>) -> City {
        let blocks = city
            .iter()
            .map(|&s| {
                s.chars()
                    .map(|c| c.to_digit(BASE_10).unwrap())
                    .collect_vec()
            })
            .collect_vec();
        let height = blocks.len() as i16;
        let width = blocks[0].len() as i16;

        City {
            blocks,
            height,
            width,
        }
    }

    fn shortest_path(&self, ultra: bool) -> u32 {
        let mut dist = HashMap::new();
        let mut heap = BinaryHeap::new();

        let start_r = Position {
            coord: (0, 0),
            direction: Direction::Right,
            steps: 0,
        };
        let start_d = Position {
            coord: (0, 0),
            direction: Direction::Down,
            steps: 0,
        };
        let goal = (self.width - 1, self.height - 1);

        dist.insert(start_r, 0);
        dist.insert(start_d, 0);
        heap.push(State {
            distance: 0,
            position: start_r,
        });
        heap.push(State {
            distance: 0,
            position: start_d,
        });

        while let Some(State { distance, position }) = heap.pop() {
            if position.coord == goal && (ultra.not() || position.steps >= 4) {
                return distance;
            }

            if &distance > dist.get(&position).unwrap_or(&u32::MAX) {
                continue;
            }

            let neighbours = match ultra {
                true => position.neighbours_ultra(self),
                false => position.neighbours_normal(self),
            };
            for next_p in neighbours {
                let next = State {
                    distance: distance + self[next_p.coord],
                    position: next_p,
                };

                if &next.distance < dist.get(&next_p).unwrap_or(&u32::MAX) {
                    heap.push(next);
                    dist.insert(next_p, next.distance);
                }
            }
        }

        panic!("Could not reach the end!")
    }
}

impl Index<Coord> for City {
    type Output = u32;

    fn index(&self, idx: Coord) -> &u32 {
        &self.blocks[idx.1 as usize][idx.0 as usize]
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct State {
    distance: u32,
    position: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.position.cmp(&other.position))
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Position {
    coord: Coord,
    direction: Direction,
    steps: u8,
}

impl Position {
    fn neighbours_normal(&self, city: &City) -> Vec<Position> {
        let mut neighbours = Vec::new();

        match self.direction {
            Direction::Right => {
                if self.steps < 3 {
                    self.try_push_right(&mut neighbours, self.steps + 1, city);
                }
                self.try_push_up(&mut neighbours, 1);
                self.try_push_down(&mut neighbours, 1, city);
            }
            Direction::Down => {
                if self.steps < 3 {
                    self.try_push_down(&mut neighbours, self.steps + 1, city);
                }
                self.try_push_left(&mut neighbours, 1);
                self.try_push_right(&mut neighbours, 1, city);
            }
            Direction::Left => {
                if self.steps < 3 {
                    self.try_push_left(&mut neighbours, self.steps + 1);
                }
                self.try_push_up(&mut neighbours, 1);
                self.try_push_down(&mut neighbours, 1, city);
            }
            Direction::Up => {
                if self.steps < 3 {
                    self.try_push_up(&mut neighbours, self.steps + 1);
                }
                self.try_push_left(&mut neighbours, 1);
                self.try_push_right(&mut neighbours, 1, city);
            }
        }

        neighbours
    }

    fn neighbours_ultra(&self, city: &City) -> Vec<Position> {
        let mut neighbours = Vec::new();

        match self.direction {
            Direction::Right => {
                if self.steps < 10 {
                    self.try_push_right(&mut neighbours, self.steps + 1, city);
                }
                if self.steps >= 4 {
                    self.try_push_up(&mut neighbours, 1);
                    self.try_push_down(&mut neighbours, 1, city);
                }
            }
            Direction::Down => {
                if self.steps < 10 {
                    self.try_push_down(&mut neighbours, self.steps + 1, city);
                }
                if self.steps >= 4 {
                    self.try_push_left(&mut neighbours, 1);
                    self.try_push_right(&mut neighbours, 1, city);
                }
            }
            Direction::Left => {
                if self.steps < 10 {
                    self.try_push_left(&mut neighbours, self.steps + 1);
                }
                if self.steps >= 4 {
                    self.try_push_up(&mut neighbours, 1);
                    self.try_push_down(&mut neighbours, 1, city);
                }
            }
            Direction::Up => {
                if self.steps < 10 {
                    self.try_push_up(&mut neighbours, self.steps + 1);
                }
                if self.steps >= 4 {
                    self.try_push_left(&mut neighbours, 1);
                    self.try_push_right(&mut neighbours, 1, city);
                }
            }
        }

        neighbours
    }

    fn try_push_right(&self, neighbours: &mut Vec<Position>, steps: u8, city: &City) {
        if self.coord.0 + 1 < city.width {
            neighbours.push(Position {
                coord: (self.coord.0 + 1, self.coord.1),
                direction: Direction::Right,
                steps,
            })
        }
    }

    fn try_push_down(&self, neighbours: &mut Vec<Position>, steps: u8, city: &City) {
        if self.coord.1 + 1 < city.height {
            neighbours.push(Position {
                coord: (self.coord.0, self.coord.1 + 1),
                direction: Direction::Down,
                steps,
            })
        }
    }

    fn try_push_left(&self, neighbours: &mut Vec<Position>, steps: u8) {
        if self.coord.0 > 0 {
            neighbours.push(Position {
                coord: (self.coord.0 - 1, self.coord.1),
                direction: Direction::Left,
                steps,
            })
        }
    }

    fn try_push_up(&self, neighbours: &mut Vec<Position>, steps: u8) {
        if self.coord.1 > 0 {
            neighbours.push(Position {
                coord: (self.coord.0, self.coord.1 - 1),
                direction: Direction::Up,
                steps,
            })
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
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
    fn day_17_part_01_sample() {
        let sample = vec![
            "2413432311323",
            "3215453535623",
            "3255245654254",
            "3446585845452",
            "4546657867536",
            "1438598798454",
            "4457876987766",
            "3637877979653",
            "4654967986887",
            "4564679986453",
            "1224686865563",
            "2546548887735",
            "4322674655533",
        ];

        assert_eq!(102, solve_1(sample));
    }

    #[test]
    fn day_17_part_01_solution() {
        let input = include_str!("../../inputs/day_17.txt").lines().collect();

        assert_eq!(791, solve_1(input));
    }

    #[test]
    fn day_17_part_02_sample() {
        let sample = vec![
            "2413432311323",
            "3215453535623",
            "3255245654254",
            "3446585845452",
            "4546657867536",
            "1438598798454",
            "4457876987766",
            "3637877979653",
            "4654967986887",
            "4564679986453",
            "1224686865563",
            "2546548887735",
            "4322674655533",
        ];

        assert_eq!(94, solve_2(sample));

        let sample = vec![
            "111111111111",
            "999999999991",
            "999999999991",
            "999999999991",
            "999999999991",
        ];

        assert_eq!(71, solve_2(sample));
    }

    #[test]
    fn day_17_part_02_solution() {
        let input = include_str!("../../inputs/day_17.txt").lines().collect();

        assert_eq!(900, solve_2(input));
    }
}
