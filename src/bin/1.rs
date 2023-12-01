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

fn concatenate_numbers(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    a.and_then(|a| b.map(|b| a.to_string() + &b.to_string())).and_then(|s| s.parse::<u32>().ok())
}

fn first(input: &str) -> u32 {
    let parsed: Vec<Vec<u32>> = input.split('\n').map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect()).collect();
    parsed.into_iter().filter_map(|v| concatenate_numbers(v.first().copied(), v.last().copied())).sum()
}

fn second(input: &str) -> u32 {
    let substrs2val: Vec<(usize, String)> = SDIGITS.iter().cloned().map(String::from).enumerate().chain((1..=9).map(|i| i.to_string()).enumerate()).map(|(i, substr)| (i + 1, substr)).collect();
    input.split('\n').filter_map(|line| {
        let frst_opt = substrs2val.iter().filter_map(|(val, substr)| line.find(substr).map(|pos| (pos, val))).min().map(|(_pos, val)| val.clone() as u32);
        let last_opt = substrs2val.iter().filter_map(|(val, substr)| line.rfind(substr).map(|pos| (pos, val))).max().map(|(_pos, val)| val.clone() as u32);
        concatenate_numbers(frst_opt, last_opt)
    }).sum()
}

/// aoc2022 runner
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