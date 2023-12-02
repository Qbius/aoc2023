use std::fs;
use clap::Parser;
use std::collections::HashMap;
use std::convert::identity;

type Res<T> = Result<T, Box<dyn std::error::Error>>;

const EXAMPLE: &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

fn parse(input: &str) -> Vec<Vec<HashMap<String, u32>>> {
    input.split('\n').filter_map(|line| {
        let marbles_str = line[line.find(':')? + 1..line.len()].trim();
        Some(marbles_str.split("; ").map(|sample| sample.split(", ").filter_map(|marble_str| {
            let pos = marble_str.find(' ')?;
            let num = marble_str[0..pos].parse::<u32>().ok()?;
            let label = String::from(&marble_str[pos + 1..marble_str.len()]);
            Some((label, num))
        }).collect()).collect())
    }).collect()
}

fn first(parsed: &Vec<Vec<HashMap<String, u32>>>) -> usize {
    let limits: HashMap<String, u32> = [(String::from("red"), 12), (String::from("green"), 13), (String::from("blue"), 14)].into_iter().collect();
    parsed.iter().enumerate().filter_map(|(i, v)| {
        match v.iter().find(|map| map.iter().filter_map(|(label, &num)| limits.get(label).map(|&limit| num > limit)).any(identity)) {
            Some(_) => None,
            None => Some(i + 1)
        }
    }).sum()
}

fn second(parsed: &Vec<Vec<HashMap<String, u32>>>) -> usize {
    let labels: Vec<String> = ["red", "green", "blue"].into_iter().map(String::from).collect();
    parsed.iter().map(|v| {
        labels.iter().map(|label| match v.iter().filter_map(|map| map.get(label)).max() {
            Some(&mx) => mx as usize,
            None => 0,
        }).product::<usize>()
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
        false => fs::read_to_string("inputs/2.txt")?,
    };
    let parsed = parse(&input);
    let first = first(&parsed);
    let second = second(&parsed);
    println!("First: {first}");
    println!("Second: {second}");
    Ok(())
}