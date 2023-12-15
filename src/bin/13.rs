use aoc::*;

fn first(input: &str) -> usize {
    input.split("\n\n").map(|s| s.split('\n').map(String::from).collect()).filter_map(score_mirror).sum()
}

fn second(input: &str) -> usize {
    input.split("\n\n").map(|s| {
        let mdgs: Vec<Vec<String>> = s.chars().enumerate().filter(|(_, c)| *c != '\n').map(|(i, c)| {
            let mut copy = s.to_string().clone();
            copy.replace_range(i..i + 1, match c {'#' => ".", '.' => "#", _ => panic!("wtf")});
            copy.split('\n').map(String::from).collect()
        }).collect();
        let pattern: Vec<String> = s.split('\n').map(String::from).collect();

        let orig_row = reflection_line(&pattern);
        let orig_col = reflection_line(&transpose(pattern.clone()));
        let orig_score = score_mirror(pattern.clone()).expect("wtf");
        mdgs.into_iter().find_map(|p| score_mirror_blacklist(p, orig_score, orig_row, orig_col)).unwrap().clone()
    }).sum()
}

fn score_mirror(pattern: Vec<String>) -> Option<usize> {
    reflection_line(&pattern).map(|i| i * 100).or_else(|| reflection_line(&transpose(pattern)))
}

fn score_mirror_blacklist(pattern: Vec<String>, blacklist: usize, orig_row: Option<usize>, orig_col: Option<usize>) -> Option<usize> {
    reflection_line_blacklist(&pattern, orig_row).map(|i| i * 100).filter(|&i| i != blacklist).or_else(|| reflection_line_blacklist(&transpose(pattern), orig_col))
}

fn reflection_line(pattern: &Vec<String>) -> Option<usize> {
    (0..pattern.len() - 1).find(|&i| pattern[0..=i].iter().rev().zip(pattern[i + 1..pattern.len()].iter()).all(|(a, b)| *a == *b)).map(|i| i + 1)
}

fn reflection_line_blacklist(pattern: &Vec<String>, blacklist: Option<usize>) -> Option<usize> {
    (0..pattern.len() - 1).filter(|&i| blacklist.map(|o| (o - 1) != i).unwrap_or(true)).find(|&i| pattern[0..=i].iter().rev().zip(pattern[i + 1..pattern.len()].iter()).all(|(a, b)| *a == *b)).map(|i| i + 1)
}

fn transpose(pattern: Vec<String>) -> Vec<String> {
    let width = pattern.first().map(|s| s.len()).unwrap_or(0);
    (0..width).map(|i| pattern.iter().filter_map(|s| s.chars().nth(i)).collect()).collect()
}

aoc!();

const EXAMPLE: &str = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";