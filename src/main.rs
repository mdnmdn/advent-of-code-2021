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
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
}

fn main() {
    let opts = Options::parse();
    match opts.day {
        AocDays::Day01 => day01::main(),
        AocDays::Day02 => day02::main(),
        AocDays::Day03 => day03::main(),
        AocDays::Day04 => day04::main(),
        AocDays::Day05 => day05::main(),
        AocDays::Day06 => day06::main(),
        AocDays::Day07 => day07::main(),
        AocDays::Day08 => day08::main(),
        AocDays::Day09 => day09::main(),
        AocDays::Day10 => day10::main(),
    };
}
