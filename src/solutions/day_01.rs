use regex::Regex;
use std::str::FromStr;

pub fn solve_1(document: Vec<String>) -> u32 {
    let re = Regex::new(r"([1-9])").unwrap();

    solve(&document, &re, &re)
}

pub fn solve_2(document: Vec<String>) -> u32 {
    let re = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_rev = Regex::new(r"([1-9]|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)").unwrap();

    solve(&document, &re, &re_rev)
}

fn solve(document: &Vec<String>, re: &Regex, re_rev: &Regex) -> u32 {
    let mut sum = 0;

    for line in document {
        let line_rev = reverse(line.as_str());

        let first = re.find(line.as_str()).unwrap().as_str();
        let string = reverse(re_rev.find(line_rev.as_str()).unwrap().as_str());
        let last = string.as_str();

        let calibration = Calibration::new(first, last);

        sum += calibration.number;
    }

    sum
}

fn reverse(string: &str) -> String {
    string.chars().rev().collect::<String>()
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
    use crate::util;

    #[test]
    fn day_01_part_01_sample() {
        let sample = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .iter()
            .map(|&s| s.to_string())
            .collect();

        assert_eq!(142, solve_1(sample))
    }

    #[test]
    fn day_01_part_01_solution() {
        let input = util::read_inputs(1);

        assert_eq!(54_331, solve_1(input))
    }

    #[test]
    fn day_01_part_02_sample() {
        let sample = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .iter()
        .map(|&s| s.to_string())
        .collect();

        assert_eq!(281, solve_2(sample))
    }

    #[test]
    fn day_01_part_02_solution() {
        let input = util::read_inputs(1);

        assert_eq!(54_518, solve_2(input))
    }
}
