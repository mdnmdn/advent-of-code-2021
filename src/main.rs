use aoc::*;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "aoc")]
struct Options {
    #[clap(subcommand)]
    day: AocDays,
}

#[derive(Subcommand, Debug)]
enum AocDays {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
}

fn main() {
    let opts = Options::parse();
    match opts.day {
        AocDays::Day01 => day01::main(),
        AocDays::Day02 => day02::main(),
        AocDays::Day03 => day03::main(),
        AocDays::Day04 => day04::main(),
        AocDays::Day05 => day05::main(),
    };
}
