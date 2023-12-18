pub fn solve_2(plan: &str) -> i64 {
    let mut prev = (0, 0);
    let mut current = (0, 0);
    let mut perimeter = 0;
    let mut triangles_sum = 0;

    for instruction in plan.lines() {
        let steps = i64::from_str_radix(
            &instruction[instruction.len() - 7..instruction.len() - 2],
            16,
        )
        .unwrap();

        match &instruction[instruction.len() - 2..instruction.len() - 1] {
            "0" => current.1 -= steps,
            "1" => current.0 += steps,
            "2" => current.1 += steps,
            "3" => current.0 -= steps,
            _ => unreachable!(),
        };

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
