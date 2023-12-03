// TODO get rid of the max usages
use std::collections::HashSet;

const BASE_10: u32 = 10;

pub fn solve_1(schematic: Vec<&str>) -> u32 {
    let schematic: Vec<Vec<char>> = schematic.iter().map(|s| s.chars().collect()).collect();

    extract_numbers(&schematic)
        .iter()
        .filter(|n| n.is_part())
        .map(|n| n.value)
        .sum()
}

pub fn solve_2(schematic: Vec<&str>) -> u32 {
    let schematic: Vec<Vec<char>> = schematic.iter().map(|s| s.chars().collect()).collect();
    let numbers = extract_numbers(&schematic);

    (0..schematic.len())
        .flat_map(|y| (0..schematic[0].len()).map(move |x| Coord { x, y }))
        .filter(|c| schematic[c.y][c.x] == '*')
        .map(|c| {
            numbers
                .iter()
                .filter(|n| n.is_adjacent_to(&c))
                .collect::<Vec<&Number>>()
        })
        .filter(|n| n.len() == 2)
        .map(|n| n[0].value * n[1].value)
        .sum()
}

fn extract_numbers(schematic: &Vec<Vec<char>>) -> Vec<Number> {
    fn push_number_if_reading<'a>(
        reading_number: &mut bool,
        numbers: &mut Vec<Number<'a>>,
        schematic: &'a Vec<Vec<char>>,
        x_s: usize,
        x_e: usize,
        y: usize,
    ) {
        if *reading_number {
            let number = Number::new(x_s, x_e, y, schematic);
            (*numbers).push(number);
            *reading_number = false;
        }
    }

    let max_x = schematic[0].len() - 1;
    let max_y = schematic.len() - 1;

    let mut numbers: Vec<Number> = Vec::new();

    let mut reading_number = false;
    let mut start = 0;

    for y in 0..=max_y {
        push_number_if_reading(
            &mut reading_number,
            &mut numbers,
            schematic,
            start,
            max_x,
            y.max(1) - 1,
        );

        for x in 0..=max_x {
            if schematic[y][x].is_digit(BASE_10) {
                if !reading_number {
                    reading_number = true;
                    start = x;
                }
            } else {
                push_number_if_reading(
                    &mut reading_number,
                    &mut numbers,
                    schematic,
                    start,
                    x.max(1) - 1,
                    y,
                );
            }
        }
    }

    push_number_if_reading(
        &mut reading_number,
        &mut numbers,
        schematic,
        start,
        max_x,
        max_y,
    );

    numbers
}

struct Number<'a> {
    value: u32,
    x_s: usize,
    x_e: usize,
    y: usize,
    schematic: &'a Vec<Vec<char>>,
}

impl<'a> Number<'a> {
    fn new(x_s: usize, x_e: usize, y: usize, schematic: &'a Vec<Vec<char>>) -> Number<'a> {
        fn value(x_s: usize, x_e: usize, y: usize, schematic: &[Vec<char>]) -> u32 {
            let mut value = 0;

            for x in x_s..=x_e {
                value = value * 10 + schematic[y][x].to_digit(BASE_10).unwrap()
            }

            value
        }

        if x_s > x_e {
            panic!("The number's end {} is before the start {}", x_e, x_s)
        }

        Number {
            value: value(x_s, x_e, y, schematic),
            x_s,
            x_e,
            y,
            schematic,
        }
    }

    fn is_part(&self) -> bool {
        let non_symbols = HashSet::from(['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
        let empty: Vec<char> = Vec::new();

        for x in (self.x_s.max(1) - 1)..=self.x_e + 1 {
            for y in (self.y.max(1) - 1)..=self.y + 1 {
                if let Some(c) = self.schematic.get(y).unwrap_or(&empty).get(x) {
                    if !non_symbols.contains(c) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn is_adjacent_to(&self, coord: &Coord) -> bool {
        let y = self.y as i32;
        let x_s = self.x_s as i32;
        let x_e = self.x_e as i32;

        let x_coord = coord.x as i32;
        let y_coord = coord.y as i32;

        (y - 1 == y_coord || y == y_coord || y + 1 == y_coord)
            && x_coord >= x_s - 1
            && x_coord <= x_e + 1
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

    #[test]
    fn day_03_part_02_sample() {
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

        assert_eq!(467_835, solve_2(sample));
    }

    #[test]
    fn day_03_part_02_solution() {
        let input = include_str!("../../inputs/day_03.txt").lines().collect();

        assert_eq!(84_289_137, solve_2(input));
    }
}
