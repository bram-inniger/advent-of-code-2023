use std::str::FromStr;
use regex::Regex;

fn solve_1(document: Vec<String>) -> u32 {
    let re = Regex::new(r"[1-9]").unwrap();
    let mut sum = 0;

    for line in document {
        let digits: Vec<&str> = re.find_iter(&*line)
            .map(|m| m.as_str())
            .collect();
        let calibration = Calibration {
            first: digits.first().unwrap().to_string(),
            last: digits.last().unwrap().to_string(),
        };

        sum += calibration.add_number();
    }

    sum
}

fn solve_2(document: Vec<String>) -> u32 {
    let re = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_rev = Regex::new(r"([1-9]|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)").unwrap();
    let mut sum = 0;

    for line in document {
        let line_rev = reverse(line.as_str());

        let first = re.find(line.as_str()).unwrap().as_str();
        let last = reverse(re_rev.find(line_rev.as_str()).unwrap().as_str());

        let calibration = Calibration {
            first: first.to_string(),
            last: last.to_string(),
        };

        sum += calibration.add_number();
    }

    sum
}

fn reverse(string: &str) -> String {
    string.chars().rev().collect::<String>()
}

struct Calibration {
    first: String,
    last: String,
}

impl Calibration {
    fn add_number(&self) -> u32 {
        Self::parse_number(self.first.as_str()) * 10 +
            Self::parse_number(self.last.as_str())
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
    use crate::util;
    use super::*;

    #[test]
    fn day_01_part_01_sample() {
        let sample = vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ]
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
        let sample = vec![
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