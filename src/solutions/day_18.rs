const DELTAS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn solve_2(plan: &str) -> i64 {
    let mut prev = (0, 0);
    let mut current = (0, 0);
    let mut perimeter = 0;
    let mut triangles_sum = 0;

    for instr in plan.lines() {
        let steps = i64::from_str_radix(&instr[instr.len() - 7..instr.len() - 2], 16).unwrap();
        let direction = instr[instr.len() - 2..instr.len() - 1]
            .chars()
            .next()
            .unwrap() as usize
            - 48; // ASCII value of '0'
        let (d_x, d_y) = DELTAS[direction];

        current = (current.0 + d_x * steps, current.1 + d_y * steps);
        perimeter += steps;
        triangles_sum += (prev.1 + current.1) * (prev.0 - current.0);
        prev = current;
    }

    i64::abs(triangles_sum / 2) + perimeter / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_18_part_02_solution() {
        let input = include_str!("../../inputs/day_18.txt");

        assert_eq!(127_844_509_405_501, solve_2(input));
    }
}
