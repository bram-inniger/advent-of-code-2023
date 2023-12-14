use std::ops::Not;

pub fn solve_1(dish: Vec<&str>) -> usize {
    Dish::new(dish).tilt_north().total_load()
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Dish {
    tiles: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
}

impl Dish {
    fn new(dish: Vec<&str>) -> Dish {
        let tiles: Vec<Vec<Tile>> = dish
            .iter()
            .map(|&s| {
                s.chars()
                    .map(|c| match c {
                        'O' => Tile::Rounded,
                        '#' => Tile::Cube,
                        '.' => Tile::Empty,
                        _ => panic!("Invalid tile: {c}"),
                    })
                    .collect()
            })
            .collect();
        let height = tiles.len();
        let width = tiles[0].len();

        Self {
            tiles,
            height,
            width,
        }
    }

    fn tilt_north(&self) -> Self {
        let mut cloned = self.clone();
        cloned.tilt_north_mut();
        cloned
    }

    fn tilt_north_mut(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if matches!(self.tiles[y][x], Tile::Rounded) {
                    let new = Self::roll_up(self, (x, y));

                    self.tiles[y][x] = Tile::Empty;
                    self.tiles[new.1][new.0] = Tile::Rounded;
                }
            }
        }
    }

    fn roll_up(&self, mut coord: (usize, usize)) -> (usize, usize) {
        loop {
            if coord.1 == 0 {
                return coord;
            }
            if matches!(self.tiles[coord.1 - 1][coord.0], Tile::Empty).not() {
                return coord;
            }
            coord.1 -= 1;
        }
    }

    fn total_load(&self) -> usize {
        (0..self.height)
            .map(|y| {
                self.tiles[y]
                    .iter()
                    .filter(|&t| matches!(t, Tile::Rounded))
                    .count()
                    * (self.height - y)
            })
            .sum()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Tile {
    Rounded,
    Cube,
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_14_part_01_sample() {
        let sample = vec![
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ];

        assert_eq!(136, solve_1(sample));
    }

    #[test]
    fn day_14_part_01_solution() {
        let input = include_str!("../../inputs/day_14.txt").lines().collect();

        assert_eq!(108_857, solve_1(input));
    }
}
