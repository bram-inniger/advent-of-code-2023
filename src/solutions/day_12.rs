use std::str::FromStr;

use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn solve_1(records: Vec<&str>) -> u64 {
    solve(records, false)
}

pub fn solve_2(records: Vec<&str>) -> u64 {
    solve(records, true)
}

fn solve(records: Vec<&str>, expand: bool) -> u64 {
    records
        .iter()
        .map(|&r| Record::new(r))
        .map(|r| r.nr_arrangements(expand))
        .sum()
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Record {
    springs: Vec<Condition>,
    criteria: Vec<usize>,
}

impl Record {
    fn new(record: &str) -> Self {
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

        Self { springs, criteria }
    }

    fn nr_arrangements(self, expand: bool) -> u64 {
        let mut memo = FxHashMap::default();
        let record = if expand { self.expand() } else { self };

        Self::nr_arrangements_rec(record, &mut memo)
    }

    fn nr_arrangements_rec(self, memo: &mut FxHashMap<Record, u64>) -> u64 {
        if let Some(v) = memo.get(&self) {
            return *v;
        }

        let record = match Self::simplify(self.clone()) {
            Ok(r) => r,
            Err(RecordStatus::Valid) => return 1,
            Err(RecordStatus::Invalid) => return 0,
        };

        let mut variant_operational = record.clone();
        variant_operational.springs[0] = Condition::Operational;

        let mut variant_damaged = record.clone();
        variant_damaged.springs[0] = Condition::Damaged;

        let nr_arrangements = Self::nr_arrangements_rec(variant_operational, memo)
            + Self::nr_arrangements_rec(variant_damaged, memo);

        memo.insert(self, nr_arrangements);

        nr_arrangements
    }

    fn simplify(self) -> Result<Self, RecordStatus> {
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

    fn expand(self) -> Self {
        let Self {
            springs: s,
            criteria: c,
        } = self;
        let u = vec![Condition::Unknown];

        let springs = [&s, &u, &s, &u, &s, &u, &s, &u, &s[..]].concat();
        let criteria = [&c, &c, &c, &c, &c[..]].concat();

        Self { springs, criteria }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
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

    #[test]
    fn day_12_part_02_sample() {
        let sample = vec![
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ];

        assert_eq!(525_152, solve_2(sample));
    }

    #[test]
    fn day_12_part_02_solution() {
        let input = include_str!("../../inputs/day_12.txt").lines().collect();

        assert_eq!(7_732_028_747_925, solve_2(input));
    }
}
