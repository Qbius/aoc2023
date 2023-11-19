use crate::common::Res;
pub type Input = Vec<i32>;
pub type Output = i32;

pub fn parse(input: String) -> Input {
    input.trim().split("\n\n").map(|elflines| elflines.trim().split('\n').map(|food| food.trim().parse::<i32>().unwrap()).sum()).collect()
}

pub fn first(elves: Input) -> Res<Output> {
    elves.into_iter().max().ok_or("Input list was empty")
}

pub fn second(mut elves: Input) -> Res<Output> {
    elves.sort();
    Ok(elves.iter().rev().take(3).sum())
}