use std::str::FromStr;
use regex::Regex;

fn solve_1(document: Vec<String>) -> u32 {
    let re = Regex::new(r"\d").unwrap();
    let mut sum = 0;

    for line in document {
        let digits: Vec<&str> = re.find_iter(&*line)
            .map(|m| m.as_str())
            .collect();
        let calibration = Calibration {
            first: digits.first().unwrap().to_string(),
            last: digits.last().unwrap().to_string(),
        };

        sum += calibration.read_number();
    }

    sum
}

struct Calibration {
    first: String,
    last: String,
}

impl Calibration {
    fn read_number(&self) -> u32 {
        u32::from_str(&*self.first).unwrap() * 10 +
            u32::from_str(&*self.last).unwrap()
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
}