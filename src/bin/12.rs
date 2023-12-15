use std::collections::HashMap;
use aoc::*;
use std::cmp::min;

#[lines]
fn first(ls: Vec<String>) -> usize {
    ls.into_iter().map(calculate).sum()
}

#[lines]
fn second(ls: Vec<String>) -> usize {
    ls.into_iter().filter_map(unfold::<5>).map(calculate).sum()
}

fn calculate(line: String) -> usize {
    let (body_str, lengths_str) = line.split_once(' ').expect("weird input");

    let body = body_str.to_string();
    let lengths: Vec<usize> = lengths_str.split(',').filter_map(|len| len.parse().ok()).collect();
    let mut cache = HashMap::new();
    arrangements(&body, &lengths, &mut cache)
}

fn arrangements<'a>(body: &'a str, lengths: &'a [usize], cache: &mut HashMap<(&'a str, &'a [usize]), usize>) -> usize {
    let key = (body, lengths);
    if !cache.contains_key(&key) {
        let res = match lengths.split_first() {
            Some((length, rest)) if body.len() >= *length => one_length(body, *length).into_iter().map(|i| arrangements(&body[min(i + *length + 1, body.len())..], rest, cache)).sum(),
            None if !body.contains('#') => 1,
            Some(_) | None => 0,
        };
        cache.insert((body, lengths), res);
    }
    cache[&key]
}

fn one_length(body: &str, length: usize) -> Vec<usize> {
    let to_index = min(body.find('#').unwrap_or(usize::MAX), body.len() - length);
    (0..=to_index).filter(|&start| !body[start..min(start + length, body.len())].contains('.') && body.chars().nth(start + length).map(|c| c != '#').unwrap_or(true)).collect()
}

fn unfold<const N: usize>(s: String) -> Option<String> {
    let (faults, lengths) = s.split_once(' ')?;
    Some(vec![faults].repeat(N).join("?") + " " + &vec![lengths].repeat(N).join(","))
}

aoc!();

const EXAMPLE: &str = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";