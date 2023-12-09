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
    let schematic: Vec<_> = schematic.iter().map(|s| s.chars().collect()).collect();
    let numbers = extract_numbers(&schematic);

    (0..schematic.len())
        .flat_map(|y| (0..schematic[0].len()).map(move |x| (x, y)))
        .filter(|(x, y)| schematic[*y][*x] == '*')
        .map(|(x, y)| {
            numbers
                .iter()
                .filter(|n| n.is_adjacent_to(x as i32, y as i32))
                .collect::<Vec<_>>()
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
        x_s: i32,
        x_e: i32,
        y: i32,
    ) {
        if *reading_number {
            let number = Number::new(x_s, x_e, y, schematic);
            (*numbers).push(number);
            *reading_number = false;
        }
    }

    let max_x = schematic[0].len() as i32 - 1;
    let max_y = schematic.len() as i32 - 1;

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
            y - 1,
        );

        for x in 0..=max_x {
            if schematic[y as usize][x as usize].is_digit(BASE_10) {
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
                    x - 1,
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
    x_s: i32,
    x_e: i32,
    y: i32,
    schematic: &'a Vec<Vec<char>>,
}

impl<'a> Number<'a> {
    fn new(x_s: i32, x_e: i32, y: i32, schematic: &'a Vec<Vec<char>>) -> Number<'a> {
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
            value: value(x_s as usize, x_e as usize, y as usize, schematic),
            x_s,
            x_e,
            y,
            schematic,
        }
    }

    fn is_part(&self) -> bool {
        let symbols = HashSet::from(['#', '$', '%', '&', '*', '+', '-', '/', '=', '@']);
        let empty: Vec<char> = Vec::new();

        ((self.y - 1)..=self.y + 1)
            .flat_map(|y| ((self.x_s - 1)..=self.x_e + 1).map(move |x| (x, y)))
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .map(|(x, y)| (x as usize, y as usize))
            .filter_map(|(x, y)| self.schematic.get(y).unwrap_or(&empty).get(x))
            .any(|c| symbols.contains(c))
    }
    fn is_adjacent_to(&self, x_n: i32, y_n: i32) -> bool {
        (self.y - 1 == y_n || self.y == y_n || self.y + 1 == y_n)
            && x_n >= self.x_s - 1
            && x_n <= self.x_e + 1
    }
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
