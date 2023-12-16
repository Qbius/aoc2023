use aoc::*;
use std::collections::HashMap;

#[grid]
fn first(gd: HashMap<(usize, usize), char>) -> i128 {
    distance_sum(parse(gd, 2)) 
}

#[grid]
fn second(gd: HashMap<(usize, usize), char>) -> i128 {
    distance_sum(parse(gd, 1000000)) 
}

fn parse(gd: HashMap<(usize, usize), char>, offset_coeff: i128) -> Vec<(i128, i128)> {
    let empty_rows: Vec<usize> = (0..=gd.ymax()).filter(|&y| (0..=gd.xmax()).filter_map(|x| gd.get(&(x, y))).all(|&c| c == '.')).collect();
    let empty_cols: Vec<usize> = (0..=gd.xmax()).filter(|&x| (0..=gd.ymax()).filter_map(|y| gd.get(&(x, y))).all(|&c| c == '.')).collect();
    gd.into_iter().filter(|(_, c)| *c == '#').map(|((x, y), _)| {
        let x_offset = empty_cols.iter().filter(|&&col| col < x).count() as i128 * (offset_coeff - 1);
        let y_offset = empty_rows.iter().filter(|&&row| row < y).count() as i128 * (offset_coeff - 1);
        (x as i128 + x_offset, y as i128 + y_offset)
    }).collect()
}

fn distance_sum(galaxies: Vec<(i128, i128)>) -> i128 {
    galaxies[0..galaxies.len() - 1].iter().enumerate().flat_map(|(i, ele1)| galaxies[i + 1..].iter().map(|ele2| (ele1.clone(), ele2.clone()))).map(|((x1, y1), (x2, y2))| (x2 - x1).abs() + (y2 - y1).abs()).sum()
}

aoc!();

const EXAMPLE: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";