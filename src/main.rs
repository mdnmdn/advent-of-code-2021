use clap::{Parser, Subcommand};

use aoc::day01;

#[derive(Parser, Debug)]
#[clap(name = "aoc")]
struct Options {
    #[clap(subcommand)]
    day: AocDays,
}

#[derive(Subcommand, Debug)]
enum AocDays {
    Day01,
}

fn main() {
    let opts = Options::parse();
    match opts.day {
        AocDays::Day01 => day01::main(),
    };
}
