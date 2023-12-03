use std::collections::HashSet;

const BASE_10: u32 = 10;

pub fn solve_1(schematic: Vec<&str>) -> u32 {
    let schematic: Vec<Vec<char>> = schematic.iter().map(|s| s.chars().collect()).collect();
    let max_x = schematic[0].len() - 1;
    let max_y = schematic.len() - 1;
    let mut sum = 0;

    let mut reading_number = false;
    let mut start = 0;

    for y in 0..=max_y {
        if reading_number {
            let number = Number::new(
                Coord { x: start, y: y - 1 },
                Coord { x: max_x, y: y - 1 },
                &schematic,
            );
            if number.is_part() {
                sum += number.value;
            }
        }

        reading_number = false;
        start = 0;

        for x in 0..=max_x {
            if schematic[y][x].is_digit(BASE_10) {
                if !reading_number {
                    reading_number = true;
                    start = x;
                }
            } else if reading_number {
                let number = Number::new(Coord { x: start, y }, Coord { x: x - 1, y }, &schematic);
                if number.is_part() {
                    sum += number.value;
                }

                reading_number = false;
                start = 0;
            }
        }
    }

    if reading_number {
        let number = Number::new(
            Coord { x: start, y: max_y },
            Coord { x: max_x, y: max_y },
            &schematic,
        );
        if number.is_part() {
            sum += number.value;
        }
    }

    sum
}

struct Number<'a> {
    value: u32,
    start: Coord,
    end: Coord,
    schematic: &'a Vec<Vec<char>>,
}

impl<'a> Number<'a> {
    fn new(start: Coord, end: Coord, schematic: &'a Vec<Vec<char>>) -> Number<'a> {
        fn value(start: &Coord, end: &Coord, schematic: &[Vec<char>]) -> u32 {
            let y = start.y;
            let mut value = 0;

            for x in start.x..=end.x {
                value = value * 10 + schematic[y][x].to_digit(BASE_10).unwrap()
            }

            value
        }

        if start.y != end.y {
            panic!(
                "The number is spanning across vertical lines: {} and {}",
                start.y, end.y
            );
        }
        if start.x > end.x {
            panic!("The number's end {} is before the start {}", end.x, start.x)
        }

        Number {
            value: value(&start, &end, schematic),
            start,
            end,
            schematic,
        }
    }

    fn is_part(&self) -> bool {
        let non_symbols = HashSet::from(['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
        let empty: Vec<char> = Vec::new();

        for x in (self.start.x.max(1) - 1)..=self.end.x + 1 {
            for y in (self.start.y.max(1) - 1)..=self.start.y + 1 {
                if let Some(c) = self.schematic.get(y).unwrap_or(&empty).get(x) {
                    if !non_symbols.contains(c) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

struct Coord {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_03_part_01_sample() {
        let sample = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        assert_eq!(4_361, solve_1(sample));
    }

    #[test]
    fn day_03_part_01_solution() {
        let input = include_str!("../../inputs/day_03.txt").lines().collect();

        assert_eq!(525_181, solve_1(input));
    }
}
