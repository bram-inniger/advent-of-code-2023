use itertools::Itertools;
use std::str::FromStr;

pub fn solve_1(records: Vec<&str>) -> usize {
    records
        .iter()
        .map(|&r| Record::new(r))
        .map(|r| r.nr_arrangements())
        .sum()
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Record {
    springs: Vec<Condition>,
    criteria: Vec<usize>,
}

impl Record {
    fn new(record: &str) -> Record {
        let split = record.split(' ').collect_vec();
        let springs = split[0]
            .chars()
            .map(|c| match c {
                '.' => Condition::Operational,
                '#' => Condition::Damaged,
                '?' => Condition::Unknown,
                _ => panic!("Invalid condition: {}", c),
            })
            .collect();
        let criteria = split[1]
            .split(',')
            .map(|s| usize::from_str(s).unwrap())
            .collect();

        Record { springs, criteria }
    }

    fn nr_arrangements(&self) -> usize {
        Self::nr_arrangements_rec(self.clone())
    }

    fn nr_arrangements_rec(self) -> usize {
        let record = match Self::simplify(self) {
            Ok(r) => r,
            Err(RecordStatus::Valid) => return 1,
            Err(RecordStatus::Invalid) => return 0,
        };

        let mut variant_operational = record.clone();
        variant_operational.springs[0] = Condition::Operational;

        let mut variant_damaged = record;
        variant_damaged.springs[0] = Condition::Damaged;

        Self::nr_arrangements_rec(variant_operational) + Self::nr_arrangements_rec(variant_damaged)
    }

    fn simplify(self) -> Result<Record, RecordStatus> {
        let Self {
            mut springs,
            mut criteria,
        } = self;

        loop {
            if springs.is_empty() {
                let status = match criteria.is_empty() {
                    true => RecordStatus::Valid,
                    false => RecordStatus::Invalid,
                };
                return Err(status);
            }

            match springs[0] {
                Condition::Damaged => {} // This condition is implicitly handled after the match statement
                Condition::Operational => {
                    springs = springs.into_iter().skip(1).collect();
                    continue;
                }
                Condition::Unknown => return Ok(Self { springs, criteria }),
            }

            if criteria.is_empty() {
                return Err(RecordStatus::Invalid);
            }

            let criterion = criteria[0];
            criteria = criteria.into_iter().skip(1).collect();

            if springs.len() < criterion {
                return Err(RecordStatus::Invalid);
            }

            if springs
                .iter()
                .take(criterion)
                .all(|s| matches!(s, Condition::Damaged) || matches!(s, Condition::Unknown))
            {
                if springs.len() > criterion {
                    if matches!(springs[criterion], Condition::Damaged) {
                        return Err(RecordStatus::Invalid);
                    } else {
                        springs = springs.into_iter().skip(criterion + 1).collect();
                    }
                } else {
                    springs = springs.into_iter().skip(criterion).collect();
                }
            } else {
                return Err(RecordStatus::Invalid);
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum RecordStatus {
    Valid,
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_12_part_01_sample() {
        let sample = vec![
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ];

        assert_eq!(21, solve_1(sample));
    }

    #[test]
    fn day_12_part_01_solution() {
        let input = include_str!("../../inputs/day_12.txt").lines().collect();

        assert_eq!(7_379, solve_1(input));
    }
}
