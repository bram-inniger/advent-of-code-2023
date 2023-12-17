use advent_of_code_2023::solutions::*;

/// Binary to more easily benchmark solutions.
///
/// How to run (requires having [hyperfine](https://crates.io/crates/hyperfine) installed):
/// ```shell
/// $ cargo build --release
/// $ hyperfine --warmup 1000 --runs 1000 'target/release/bench'
/// ```
fn main() {
    let input = include_str!("../../inputs/day_17.txt").lines().collect();

    assert_eq!(791, day_17::solve_1(input));

    let input = include_str!("../../inputs/day_17.txt").lines().collect();

    assert_eq!(900, day_17::solve_2(input));
}
