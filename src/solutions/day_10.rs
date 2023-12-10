use std::ops::Index;

pub fn solve_1(maze: Vec<&str>) -> u32 {
    Maze::new(maze).loop_length() / 2
}

#[derive(Debug)]
struct Maze {
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    fn new(maze: Vec<&str>) -> Maze {
        let tiles: Vec<_> = maze
            .iter()
            .map(|l| l.chars().map(Tile::new).collect())
            .collect();
        Maze { tiles }
    }

    fn loop_length(&self) -> u32 {
        let start = Self::find_start(self);

        let mut prev = start;
        let mut current = Self::next(self, start, start);

        let mut steps = 1;

        while current != start {
            let next = Self::next(self, prev, current);

            prev = current;
            current = next;

            steps += 1;
        }

        steps
    }

    fn find_start(&self) -> (usize, usize) {
        (0..self.tiles.len())
            .flat_map(|y| (0..self.tiles[0].len()).map(move |x| (x, y)))
            .find(|&(x, y)| matches!(self[(x, y)], Tile::Start))
            .unwrap()
    }

    fn next(&self, prev: (usize, usize), current: (usize, usize)) -> (usize, usize) {
        *Self::neighbours(self, current)
            .iter()
            .find(|&&n| n != prev)
            .unwrap()
    }

    fn neighbours(&self, tile: (usize, usize)) -> Vec<(usize, usize)> {
        match self[tile] {
            Tile::NorthSouth => vec![(tile.0, tile.1 - 1), (tile.0, tile.1 + 1)],
            Tile::EastWest => vec![(tile.0 + 1, tile.1), (tile.0 - 1, tile.1)],
            Tile::NorthEast => vec![(tile.0, tile.1 - 1), (tile.0 + 1, tile.1)],
            Tile::NorthWest => vec![(tile.0, tile.1 - 1), (tile.0 - 1, tile.1)],
            Tile::SouthWest => vec![(tile.0, tile.1 + 1), (tile.0 - 1, tile.1)],
            Tile::SouthEast => vec![(tile.0, tile.1 + 1), (tile.0 + 1, tile.1)],
            Tile::Ground => vec![],
            Tile::Start => Self::start_neighbours(self, tile),
        }
    }

    fn start_neighbours(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let height = self.tiles.len();
        let width = self.tiles[0].len();
        let mut neighbours = Vec::new();

        if start.0 >= 1 {
            if let Tile::EastWest | Tile::NorthEast | Tile::SouthEast =
                &self[(start.0 - 1, start.1)]
            {
                neighbours.push((start.0 - 1, start.1))
            }
        }

        if start.1 >= 1 {
            if let Tile::NorthSouth | Tile::SouthWest | Tile::SouthEast =
                &self[(start.0, start.1 - 1)]
            {
                neighbours.push((start.0, start.1 - 1))
            }
        }

        if start.0 < width - 1 {
            if let Tile::EastWest | Tile::NorthWest | Tile::SouthWest =
                &self[(start.0 + 1, start.1)]
            {
                neighbours.push((start.0 + 1, start.1))
            }
        }

        if start.1 < height - 1 {
            if let Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest =
                &self[(start.0, start.1 + 1)]
            {
                neighbours.push((start.0, start.1 + 1))
            }
        }

        neighbours
    }
}

impl Index<(usize, usize)> for Maze {
    type Output = Tile;

    // the index is formatted (x, y)
    fn index(&self, idx: (usize, usize)) -> &Tile {
        &self.tiles[idx.1][idx.0]
    }
}

#[derive(Debug)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Tile {
    fn new(tile: char) -> Tile {
        match tile {
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("Received invalid tile: {tile}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_10_part_01_sample() {
        let sample = vec!["-L|F7", "7S-7|", "L|7||", "-L-J|", "L|-JF"];

        assert_eq!(4, solve_1(sample));

        let sample = vec!["7-F7-", ".FJ|7", "SJLL7", "|F--J", "LJ.LJ"];

        assert_eq!(8, solve_1(sample));
    }

    #[test]
    fn day_10_part_01_solution() {
        let input = include_str!("../../inputs/day_10.txt").lines().collect();

        assert_eq!(7_107, solve_1(input));
    }
}
