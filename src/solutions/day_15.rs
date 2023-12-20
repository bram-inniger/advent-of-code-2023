use std::str::FromStr;
use std::usize;

use indexmap::IndexMap;

pub fn solve_1(init_sequence: &str) -> usize {
    init_sequence.split(',').map(|s| hash(s) as usize).sum()
}

pub fn solve_2(init_sequence: &str) -> usize {
    let mut facility = Facility::new();

    for step in init_sequence.split(',').map(Step::new) {
        facility.install(step);
    }

    facility.focus_power()
}

fn hash(step: &str) -> u8 {
    let mut value = 0;

    for x in step.chars() {
        value += x as u16;
        value *= 17;
        value %= 256;
    }

    value as u8
}

#[derive(Debug)]
struct Facility<'a> {
    boxes: Vec<IndexMap<&'a str, usize>>,
}

impl<'a> Facility<'a> {
    fn new() -> Facility<'a> {
        let mut boxes: Vec<IndexMap<&'a str, usize>> = Vec::new();
        (0..256).for_each(|_| boxes.push(IndexMap::new()));

        Facility { boxes }
    }

    fn install(&mut self, step: Step<'a>) {
        let label = match step {
            Step::Add { ref lens } => lens.label,
            Step::Remove { label } => label,
        };

        match step {
            Step::Add { lens } => {
                self.boxes[hash(label) as usize].insert(label, lens.focal_length);
            }
            Step::Remove { label: _ } => {
                self.boxes[hash(label) as usize].shift_remove_entry(label);
            }
        }
    }

    fn focus_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(box_nr, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(move |(lens_slot, (_, focal_length))| {
                        (box_nr + 1) * (lens_slot + 1) * (focal_length)
                    })
            })
            .sum()
    }
}

#[derive(Debug)]
enum Step<'a> {
    Add { lens: Lens<'a> },
    Remove { label: &'a str },
}

impl<'a> Step<'a> {
    fn new(step: &'a str) -> Step<'a> {
        if step.contains('=') {
            let split: Vec<_> = step.split('=').collect();
            let lens = Lens {
                label: split[0],
                focal_length: usize::from_str(split[1]).unwrap(),
            };
            Step::Add { lens }
        } else if step.contains('-') {
            let label = step.trim_end_matches('-');
            Step::Remove { label }
        } else {
            panic!("Invalid step: {step}")
        }
    }
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_15_part_01_sample() {
        let sample = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(1_320, solve_1(sample));
    }

    #[test]
    fn day_15_part_01_solution() {
        let input = include_str!("../../inputs/day_15.txt");

        assert_eq!(514_281, solve_1(input));
    }

    #[test]
    fn day_15_part_02_sample() {
        let sample = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(145, solve_2(sample));
    }

    #[test]
    fn day_15_part_02_solution() {
        let input = include_str!("../../inputs/day_15.txt");

        assert_eq!(244_199, solve_2(input));
    }
}
