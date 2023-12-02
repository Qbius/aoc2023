use std::fs;
use clap::Parser;

type Res<T> = Result<T, Box<dyn std::error::Error>>;

const SDIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const EXAMPLE: &str = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

fn first(input: &str) -> u32 {
    let parsed: Vec<Vec<u32>> = input.split('\n').map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect()).collect();
    parsed.into_iter().filter_map(|v| Some(v.first()? * 10 + v.last()?)).sum()
}

fn second(input: &str) -> usize {
    let substrs2val: Vec<(usize, String)> = SDIGITS.iter().cloned().map(String::from).enumerate().chain((1..=9).map(|i| i.to_string()).enumerate()).map(|(i, substr)| (i + 1, substr)).collect();
    input.split('\n').filter_map(|line| {
        let frst_opt = substrs2val.iter().filter_map(|(val, substr)| line.find(substr).map(|pos| (pos, val))).min().map(|(_pos, val)| val);
        let last_opt = substrs2val.iter().filter_map(|(val, substr)| line.rfind(substr).map(|pos| (pos, val))).max().map(|(_pos, val)| val);
        Some(frst_opt? * 10 + last_opt?)
    }).sum()
}

/// aoc2023 runner
#[derive(Parser)]
struct Args {
   /// If set, run the day's example
   #[arg(short, long, action)]
   example: bool,
}

fn main() -> Res<()> {
    let args = Args::parse();
    let input = match args.example {
        true => String::from(EXAMPLE),
        false => fs::read_to_string("inputs/1.txt")?,
    };
    let first = first(input.trim());
    let second = second(input.trim());
    println!("First: {first}");
    println!("Second: {second}");
    Ok(())
}