use itertools::Itertools;
use std::str::FromStr;

pub fn solve_1(init_sequence: &str) -> u32 {
    init_sequence.split(',').map(|s| hash(s) as u32).sum()
}

pub fn solve_2(init_sequence: &str) -> u32 {
    let mut facility = Facility::new();

    for step in init_sequence.split(',').map(Step::new) {
        facility.install(step);
    }

    facility.focus_power()
}

fn hash(step: &str) -> u8 {
    let mut value = 0;

    for x in step.chars() {
        value += x as u32;
        value *= 17;
        value %= 256;
    }

    value as u8
}

#[derive(Debug)]
struct Facility<'a> {
    boxes: Vec<Vec<Lens<'a>>>,
}

impl<'a> Facility<'a> {
    fn new() -> Facility<'a> {
        let mut boxes: Vec<Vec<Lens<'a>>> = Vec::new();
        (0..256).for_each(|_| boxes.push(Vec::new()));

        Facility { boxes }
    }

    fn install(&mut self, step: Step<'a>) {
        let label = match step {
            Step::Add { ref lens } => lens.label,
            Step::Remove { label } => label,
        };

        let lenses = &mut self.boxes[hash(label) as usize];
        let existing_lens = lenses.iter().find_position(|&l| l.label == label);

        match step {
            Step::Add { lens } => match existing_lens {
                None => lenses.push(lens),
                Some((idx, _)) => lenses[idx] = lens,
            },
            Step::Remove { label: _ } => match existing_lens {
                None => {}
                Some((idx, _)) => {
                    lenses.remove(idx);
                }
            },
        }
    }

    fn focus_power(&self) -> u32 {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(box_nr, lenses)| {
                lenses.iter().enumerate().map(move |(lens_slot, lens)| {
                    ((box_nr + 1) as u32) * ((lens_slot + 1) as u32) * (lens.focal_length as u32)
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
                focal_length: u8::from_str(split[1]).unwrap(),
            };
            Step::Add { lens }
        } else if step.contains('-') {
            Step::Remove {
                label: step.trim_end_matches('-'),
            }
        } else {
            panic!("Invalid step: {step}")
        }
    }
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
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
