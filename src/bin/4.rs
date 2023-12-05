use aoc::*;
use std::collections::HashMap;

const EXAMPLE: &str = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[lines]
fn parse_cards(lines: Vec<String>) -> Vec<usize> {
    lines.into_iter().filter_map(|line| {
        let numbers_str = line.split(':').nth(1)?;
        let split: Vec<_> = numbers_str.split('|').collect();
        let winning: Vec<_> = split.first()?.split(' ').filter_map(|e| e.parse::<usize>().ok()).collect();
        let woncount = split.last()?.split(' ').filter_map(|e| e.parse::<usize>().ok()).filter(|num| winning.contains(num)).count();
        Some(woncount)
    }).collect()
}

fn first(woncounts: Vec<usize>) -> usize {
    woncounts.into_iter().filter(|&num| num > 0).map(|num| (2 as usize).pow((num - 1) as u32)).sum()
}

struct Scratcher {
    woncounts: Vec<usize>,
    cache: HashMap<usize, usize>,
}

impl Scratcher {
    fn new(woncounts: Vec<usize>) -> Self {
        Scratcher {
            woncounts,
            cache: HashMap::new(),
        }
    }

    fn scratch_unders(&mut self, which: usize) -> usize {
        if !self.cache.contains_key(&which) {
            let woncount = self.woncounts[which];
            let mut sum = woncount;
            for new_which in which + 1..=which + woncount {
                sum += self.scratch_unders(new_which);
            }
            self.cache.insert(which, sum);
        }
        self.cache.get(&which).unwrap().clone()
    }

    fn scratch_all(&mut self) -> usize {
        let mut sum = self.woncounts.len();
        for which in 0..self.woncounts.len() {
            sum += self.scratch_unders(which);
        }
        sum
    }
}

fn second(woncounts: Vec<usize>) -> usize {
    let mut scratcher = Scratcher::new(woncounts);
    scratcher.scratch_all()
}

aoc!(parse_cards);