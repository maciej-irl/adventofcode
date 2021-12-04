#![allow(dead_code)]
#![feature(vec_retain_mut)]

mod aoc;
mod day01;
mod day02;
mod day03;
mod day04;

use pico_args::Arguments;
use std::error::Error;
use std::fs::read_to_string;
use std::io::{self, prelude::*};

fn run_day(day: u32, input: String) {
    use aoc::run;

    match day {
        1 => run(day01::solve, input),
        2 => run(day02::solve, input),
        3 => run(day03::solve, input),
        4 => run(day04::solve, input),
        _ => panic!("day not implemented"),
    }
}

fn day_input_path(day: u32) -> String {
    format!("inputs/day{day:02}.txt")
}

fn read_stdin() -> io::Result<String> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Arguments::from_env();
    if args.contains("--help") || args.contains("-h") {
        eprint!("aoc [--help] [<day> <input>]");
        return Ok(());
    }

    if let Some(day) = args.opt_free_from_str()? {
        let input = match args.opt_free_from_str::<String>()?.as_deref() {
            Some("-") => read_stdin()?,
            Some(path) => read_to_string(path)?,
            None => read_to_string(&day_input_path(day))?,
        };
        run_day(day, input);
    } else {
        for day in 1..=4 {
            run_day(day, read_to_string(&day_input_path(day))?);
        }
    }

    Ok(())
}
