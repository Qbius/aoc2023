use std::collections::HashMap;

use aoc::*;
use regex::Regex;
use std::cmp::max;

const EXAMPLE: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

fn get_info(lines: Vec<String>) -> Option<(Vec<(usize, usize, usize, usize)>, HashMap<(usize, usize), char>)> {
    let re_numbers = Regex::new(r"\d+").ok()?;
    let re_symbols = Regex::new(r"[^\.\d]").ok()?;
    let (all_numbers, all_symbols): (Vec<_>, Vec<_>) = lines.into_iter().enumerate().map(|(y, line)| {
        let numbers: Vec<_> = re_numbers.find_iter(&line).filter_map(|matched| Some((matched.start(), y, matched.len(), matched.as_str().parse::<usize>().ok()?))).collect();
        let symbols: Vec<_> = re_symbols.find_iter(&line).filter_map(|matched| Some(((matched.start(), y), matched.as_str().chars().nth(0)?))).collect();
        (numbers, symbols)
    }).unzip();
    let numbers: Vec<_> = all_numbers.into_iter().flat_map(Vec::into_iter).collect();
    let symbols: HashMap<(usize, usize), char> = all_symbols.into_iter().flat_map(Vec::into_iter).collect();
    Some((numbers, symbols))
}

fn adjacent(point: (usize, usize), len: usize) -> Vec<(usize, usize)> {
    let (x, y) = point;
    (max(y, 1) - 1..=y + 1).flat_map(move |b| (max(x, 1) - 1..=x + len).map(move |a| (a, b))).collect()
}

#[lines]
fn first(lines: Vec<String>) -> usize {
    let (numbers, symbols) = get_info(lines).expect("Something went wrong with the input");
    numbers.into_iter().filter_map(|(x, y, len, num)| {
        match adjacent((x, y), len).iter().any(|point| symbols.contains_key(point)) {
            true => Some(num),
            false => None,
        }
    }).sum()
}

#[lines]
fn second(lines: Vec<String>) -> usize {
    let (numbers, symbols) = get_info(lines).expect("Something went wrong with the input");
    symbols.into_iter().filter(|&(_, symbol)| symbol == '*').filter_map(|(point, _)| {
        let adj = adjacent(point, 1);
        let adj_numbers: Vec<_> = numbers.iter().filter(|(x, y, len, _)| (x.clone()..x.clone() + len.clone()).map(|a| (a, y.clone())).any(|p| adj.contains(&p))).map(|(_, _, _, num)| num.clone()).collect();
        if adj_numbers.len() == 2 {
            Some(adj_numbers.first()? * adj_numbers.last()?)
        }
        else {
            None
        }
    }).sum()
}

aoc!();