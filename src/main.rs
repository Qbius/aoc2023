mod common;
mod aoc;

use clap::Parser;

/// aoc2022 runner
#[derive(Parser)]
struct Args {
   /// Which aoc2022 day to run, from 1 to 25
   day: usize,

   /// If set, run the day's example
   #[arg(short, long, action)]
   example: bool,
}

fn main() {
    let args = Args::parse();
    let input_directory = match args.example {
        true => "examples",
        false => "inputs",
    };
    aoc::day(args.day, input_directory.to_string());
}