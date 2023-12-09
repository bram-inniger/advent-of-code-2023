use regex::Regex;
use std::str::FromStr;

pub fn solve_1(document: Vec<&str>) -> u32 {
    let re = Regex::new(r"([1-9])").unwrap();

    solve(&document, &re, &re)
}

pub fn solve_2(document: Vec<&str>) -> u32 {
    let re = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_rev = Regex::new(r"([1-9]|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)").unwrap();

    solve(&document, &re, &re_rev)
}

fn solve(document: &[&str], re: &Regex, re_rev: &Regex) -> u32 {
    document
        .iter()
        .map(|&line| {
            let first = re.find(line).unwrap().as_str();
            let last = &reverse(re_rev.find(&reverse(line)).unwrap().as_str());

            Calibration::new(first, last)
        })
        .map(|calibration| calibration.number)
        .sum()
}

fn reverse(string: &str) -> String {
    string.chars().rev().collect()
}

struct Calibration {
    number: u32,
}

impl Calibration {
    fn new(first: &str, last: &str) -> Calibration {
        let number = Self::parse_number(first) * 10 + Self::parse_number(last);
        Calibration { number }
    }

    fn parse_number(number: &str) -> u32 {
        match number {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            other => u32::from_str(other).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_01_part_01_sample() {
        let sample = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        assert_eq!(142, solve_1(sample));
    }

    #[test]
    fn day_01_part_01_solution() {
        let input = include_str!("../../inputs/day_01.txt").lines().collect();

        assert_eq!(54_331, solve_1(input));
    }

    #[test]
    fn day_01_part_02_sample() {
        let sample = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        assert_eq!(281, solve_2(sample));
    }

    #[test]
    fn day_01_part_02_solution() {
        let input = include_str!("../../inputs/day_01.txt").lines().collect();

        assert_eq!(54_518, solve_2(input));
    }
}
