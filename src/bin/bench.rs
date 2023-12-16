use advent_of_code_2023::solutions::*;

/// Binary to more easily benchmark solutions.
///
/// How to run (requires having [hyperfine](https://crates.io/crates/hyperfine) installed):
/// ```shell
/// $ cargo build --release
/// $ hyperfine --warmup 1000 --runs 1000 'target/release/bench'
/// ```
fn main() {
    let input = include_str!("../../inputs/day_16.txt").lines().collect();

    assert_eq!(8_383, day_16::solve_2(input));
}
