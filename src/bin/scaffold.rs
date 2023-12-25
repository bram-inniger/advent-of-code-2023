use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::ops::Not;
use std::str::FromStr;
use std::{env, process};

use chrono::Datelike;

/// Binary to scaffold code for a new Advent of Code day.
///
/// # How to run
///
/// This will scaffold all code for the day it is ran on:
/// ```shell
/// $ cargo run --bin scaffold
/// ```
///
/// This will scaffold all code for "Day 07":
/// ```shell
/// $ cargo run --bin scaffold -- 7
/// ```
///
/// Alternatively the binary can be built and called directly:
/// ```shell
/// $ cargo build --release
/// $ ./target/release/scaffold 7
/// ```
///
/// Optionally an environment variable `OVERWRITE` can be set,
/// this will overwrite contents of existing files,
/// normally existing files would throw an error.
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    day: u8,
    overwrite: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        let day = if args.len() >= 2 {
            u8::from_str(&args[1])
        } else {
            Ok(chrono::prelude::Utc::now().day() as u8)
        };
        if day.is_err() {
            return Err("invalid day");
        }
        let day = day.unwrap();

        let overwrite = env::var("OVERWRITE").is_ok();

        Ok(Config { day, overwrite })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let day = format!("day_{:0>2}", config.day);

    write_file(&config, &format!("./inputs/{}.txt", day), "")?;
    write_file(&config, &format!("./problems/{}.txt", day), "")?;
    write_file(
        &config,
        &format!("./src/solutions/{}.rs", day),
        &TEMPLATE.replace("{}", &day),
    )?;

    append_file("./src/solutions.rs", &format!("pub mod {};\n", day))?;

    Ok(())
}

fn write_file(config: &Config, path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    File::options()
        .create_new(config.overwrite.not())
        .write(true)
        .truncate(true)
        .open(path)?
        .write_all(content.as_ref())?;

    Ok(())
}

fn append_file(path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    File::options()
        .write(true)
        .append(true)
        .open(path)?
        .write_all(content.as_ref())?;

    Ok(())
}

const TEMPLATE: &str = "\
// use std::str::FromStr;

// use regex::Regex;

pub fn solve_1(X: &[&str]) -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn {}_part_01_sample() {
        let sample = vec![
            \"\",
            \"\",
            \"\",
            \"\",
            \"\",
            \"\",
            \"\",
            \"\",
            \"\",
            \"\",
        ];

        assert_eq!(42, solve_1(&sample));
    }

    #[test]
    fn {}_part_01_solution() {
        let input = include_str!(\"../../inputs/{}.txt\")
            .lines()
            .collect_vec();

        assert_eq!(0, solve_1(&input));
    }
}
";
