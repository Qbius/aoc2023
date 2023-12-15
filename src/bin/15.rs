use std::collections::HashMap;

use aoc::*;

fn first(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn second(input: &str) -> usize {
    let mut lens = HashMap::<usize, Vec<(&str, usize)>>::new();
    input.split(',').for_each(|s| {
        let (label, value_perhaps) = s.split_once('=').or(s.split_once('-')).expect("wtf");
        let entry = lens.entry(hash(label)).or_default();
        match (value_perhaps.parse().ok(), entry.iter().position(|(its_label, _)| *its_label == label)) {
            (None, _) => *entry = entry.iter().cloned().filter(|(its_label, _)| *its_label != label).collect(),
            (Some(val), Some(i)) => entry[i] = (label, val),
            (Some(val), None) => entry.push((label, val)),
        }
    });
    lens.into_iter().map(|(boxn, lenss)| lenss.into_iter().enumerate().map(|(i, (_, n))| (boxn + 1) * (i + 1) * n).sum::<usize>()).sum()
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| (17 * (acc + c as usize)) % 256)
}

aoc!();

const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";