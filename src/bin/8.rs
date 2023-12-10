use aoc::*;
use std::collections::HashMap;

const EXAMPLE: &str = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

fn first((directions, nodes): (String, HashMap<String, (String, String)>)) -> usize {
    let (_, z) = find_cycle(&directions, &nodes, String::from("AAA")).expect("no cycle no play");
    z + 1
}

fn second((directions, nodes): (String, HashMap<String, (String, String)>)) -> usize {
    let cycles: Vec<usize> = nodes.keys().cloned().filter(|k| match k.chars().last() {Some('A') => true, _ => false}).into_iter().map(|start| find_cycle(&directions, &nodes, start).expect("No cycle no play")).map(|(cycle, _)| cycle).collect();
    match cycles.into_iter().reduce(lcm) {
        Some(res) => res,
        None => panic!("No cycles"),
    }
}

fn find_cycle(directions: &String, nodes: &HashMap<String, (String, String)>, start: String) -> Option<(usize, usize)> {
    let mut z: Option<usize> = None;
    let mut node = start.to_owned();
    let mut visited = HashMap::<(String, usize), usize>::new();
    
    let mut iter = directions.chars().cycle().enumerate();
    loop {
        let (i, dir) = iter.next()?;
        let key = (node.to_owned(), i % directions.len());
        if let Some(&prev) = visited.get(&key) {
            return Some((i - prev, z.expect("no z")))   
        }
        else {
            visited.insert(key, i);
        }

        let (left, right) = nodes.get(&node)?;
        node = match dir {
            'L' => left.to_owned(),
            'R' => right.to_owned(),
            _ => panic!("weird direction"),
        };
        if node.chars().last()? == 'Z' {
            z = Some(i);
        }
    }
}

#[lines]
fn parse(ls: Vec<String>) -> (String, HashMap<String, (String, String)>) {
    let directions = ls[0].to_string();
    (directions, ls[2..].iter().map(|s| (s[0..3].to_string(), (s[7..10].to_string(), s[12..15].to_string()))).collect())
}

aoc!(parse);