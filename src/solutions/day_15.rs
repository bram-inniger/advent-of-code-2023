pub fn solve_1(init_sequence: &str) -> u32 {
    init_sequence.split(',').map(|s| hash(s) as u32).sum()
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
}
