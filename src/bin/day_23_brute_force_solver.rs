use advent_of_code_2023::solutions::day_23;
use itertools::Itertools;
use std::thread;

fn main() {
    let builder = thread::Builder::new().stack_size(4 * 1024 * 1024 * 1024); // 4 GB
    let handler = builder
        .spawn(|| {
            let input = include_str!("../../inputs/day_23.txt")
                .lines()
                .collect_vec();

            println!("{}", day_23::solve_2(&input));
        })
        .unwrap();
    handler.join().unwrap();
}
