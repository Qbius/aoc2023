use std::collections::HashMap;
use counter::Counter;
use itertools::Itertools;

use aoc::*;
use regex::Regex;

type Workflows = HashMap<String, Vec<Check>>;
type Part = HashMap<char, usize>;
type Parts = Vec<Part>;

#[derive(Clone, Debug)]
enum Check {
    Normal(char, char, usize, String),
    Fallback(String),
}

impl Check {
    fn test_part(&self, part: &Part) -> Option<String> {
        match self {
            Check::Normal(prop, sign, n, next) => {
                if self.test_scalar(part.get(prop)?.clone()) {
                    Some(next.clone())
                }
                else {
                    None
                }
            }
            Check::Fallback(next) => {
                Some(next.clone())
            }
        } 
    }

    fn test_scalar(&self, scalar: usize) -> bool {
        match self {
            Check::Normal(_, sign, n, _) => match sign {
                '>' => scalar > *n,
                '<' => scalar < *n,
                _ => panic!("wtf"),
            },
            Check::Fallback(_) => true,
        }
    }

    fn destination(&self) -> String {
        match self {
            Check::Normal(_, _, _, next) => next.clone(),
            Check::Fallback(next) => next.clone(),
        }
    }
}

fn first((workflows, parts): (Workflows, Parts)) -> usize {
    parts.into_iter().filter(|part| accepted(part, &workflows)).map(|part| part.values().into_iter().sum::<usize>()).sum()
}

fn accepted(part: &Part, workflows: &Workflows) -> bool {
    let mut next = String::from("in");
    loop {
        match workflows.get(&next).expect("no such workflow").iter().find_map(|check| check.test_part(&part)) {
            Some(r) if r == String::from("A") => {
                return true;
            }
            Some(r) if r == String::from("R") => {
                return false;
            }
            Some(label) => {
                next = label;
            }
            None => {
                panic!("ran out of workflows");
            }
        }
    }
}

fn second((workflows, parts): (Workflows, Parts)) -> usize {
    // let A = String::from("A");
    // println!("{:?}", possible_paths(A, &workflows));
    // 0
    let a = String::from("A");
    let all_ranges = workflows
        .iter()
        .filter_map(|(label, checks)| {
            match checks.iter().any(|check| check.destination() == a) {
                true => Some((label.clone(), checks.iter().take_while_inclusive(|check| check.destination() != a).cloned().collect_vec())),
                false => None,
            }
        })
        .map(|(label, final_checks)| {
            let last = final_checks.len() - 1;
            find_path_to(label, &workflows).unwrap().into_iter().chain(final_checks.into_iter().enumerate().map(|(i, check)| (check, i == last))).collect_vec()
        })
        .map(analyze_check_path)
        .collect_vec()
    ;
//    println!("{all_ranges:?}");
    let possibilities_dumb = all_ranges.iter().map(|ranges| ranges.iter().map(Vec::len).product::<usize>()).sum::<usize>();
    println!("{possibilities_dumb:?}");
    let intersection: usize = (0..=3usize).map(|i| (1..=4000usize).map(|n| std::cmp::max(all_ranges.iter().filter(|ranges| ranges[i].contains(&n)).count(), 2) - 1).product::<usize>()).sum();
    possibilities_dumb - intersection
    // 167409079868000
    // 39201891528000

}

struct Node {
    children: Vec<Box<Node>>,
}

 

fn find_path_to(label: String, workflows: &Workflows) -> Option<Vec<(Check, bool)>> {
    let (prev, checks_to) = workflows.iter().find(|(_, checks)| checks.iter().any(|check| check.destination() == label)).map(|(l, cs)| (l.clone(), cs.iter().take_while_inclusive(|check| check.destination() != label).cloned().collect_vec()))?;
    let last = checks_to.len() - 1;
    let checks_to_labeled = checks_to.into_iter().enumerate().map(|(i, check)| (check, i == last)).collect_vec();
    match find_path_to(prev, workflows) {
        Some(prev_checks) => { 
            Some(prev_checks.into_iter().chain(checks_to_labeled.into_iter()).collect())
        }
        None => {
            Some(checks_to_labeled)
        }
    }
}

fn analyze_check_path(path: Vec<(Check, bool)>) -> Vec<Vec<usize>> {
    ['x', 'm', 'a', 's'].into_iter().map(|prop| {
        let appliable_checks = path.iter().filter_map(|(check, _)| match check {
            Check::Normal(c, _, _, _) if *c == prop => Some(check),
            _ => None,
        }).collect_vec();
        (1..=4000usize).filter(|&i| appliable_checks.iter().all(|check| check.test_scalar(i))).collect()
    }).collect()
}

// fn possible_paths<'a>(dst: String, workflows: Workflows) -> Vec<Vec<Check>> {
//     workflows.iter().filter(|(_, checks)| checks.iter().cloned().any(|check| check.destination() == dst)).map(|(label, checks)| {
//         possible_paths(label.clone(), workflows).into_iter().map(|check_line| {
//             check_line.into_iter().chain(checks.into_iter().take_while_inclusive(|check| check.destination() != dst)).collect()
//         }).collect()
//     }).collect()
// }

fn parse(input: &str) -> (Workflows, Parts) {
    parse_details(input).expect("weird input")
}

fn parse_details(input: &str) -> Option<(Workflows, Parts)> {
    let workflow_re = Regex::new(r"([a-z]+)\{([^\}]+)\}").ok()?;
    let part_re = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}").ok()?;
    let rule_re = Regex::new(r"(x|m|a|s)(>|<)([0-9]+)\:([a-zA-Z]+)").ok()?;
    let (workflows_str, parts_str) = input.split_once("\n\n")?;
    
    let workflows: Workflows = workflows_str.split('\n').filter_map(|s| workflow_re.captures(s)).filter_map(|wf_capture| {
        let rules_vec = wf_capture[2].split(',').collect_vec();
        let (last_label, rulesets) = rules_vec.split_last()?;
        let mut rules_defs: Vec<Check> = rulesets.into_iter().filter_map(|rulestr| rule_re.captures(rulestr)).filter_map(|rule_capture| Some(Check::Normal(rule_capture[1].chars().nth(0)?, rule_capture[2].chars().nth(0)?, rule_capture[3].parse::<usize>().ok()?, String::from(&rule_capture[4])))).collect();
        rules_defs.push(Check::Fallback(String::from(*last_label)));
        Some((String::from(&wf_capture[1]), rules_defs))
    }).collect();

    let parts: Parts = parts_str.split('\n').filter_map(|p| part_re.captures(p)).filter_map(|part_capture| Some([('x', part_capture[1].parse::<usize>().ok()?), ('m', part_capture[2].parse::<usize>().ok()?), ('a', part_capture[3].parse::<usize>().ok()?), ('s', part_capture[4].parse::<usize>().ok()?)].into_iter().collect())).collect();
    Some((workflows, parts))
}



aoc!(parse);

const EXAMPLE: &str = "
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";