use aoc::*;

const EXAMPLE: &str = "
hehe: 7
hehe: 2
hehe: 0";

fn first(input: &str) -> i32 {
    input.len() as i32
}

#[lines]
fn second(lines: Vec<String>) -> i32 {
    lines.len() as i32
}

aoc!();