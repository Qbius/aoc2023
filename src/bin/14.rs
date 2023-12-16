use std::collections::HashMap;
use counter::Counter;
use aoc::*;

fn first((rounds, statics, xmax, ymax): (Vec<(usize, usize)>, Vec<(usize, usize)>, usize, usize)) -> usize {
    let new_rounds = shake(rounds, &statics, xmax, ymax, Up);
    let height = ymax + 1;
    new_rounds.into_iter().map(|(_, y)| height - y).sum()
}

fn second((rounds, statics, xmax, ymax): (Vec<(usize, usize)>, Vec<(usize, usize)>, usize, usize)) -> usize {
    let (start, end, new_rounds) = find_repeat(rounds, &statics, xmax, ymax);
    let remaining_cycles = (1000000000 - start) % (end - start);
    let end_rounds = (0..remaining_cycles).fold(new_rounds, |rs, _| cycle(rs, &statics, xmax, ymax));
    let height = ymax + 1;
    end_rounds.into_iter().map(|(_, y)| height - y).sum()
}

const CYCLE: [Direction; 4] = [Up, Left, Down, Right];
fn cycle(rounds: Vec<(usize, usize)>, statics: &Vec<(usize, usize)>, xmax: usize, ymax: usize) -> Vec<(usize, usize)> {
    CYCLE.iter().fold(rounds, |rs, &dir| shake(rs, &statics, xmax, ymax, dir))
}

fn shake(rounds: Vec<(usize, usize)>, statics: &Vec<(usize, usize)>, xmax: usize, ymax: usize, dir: Direction) -> Vec<(usize, usize)> {
    let counts: Counter<(usize, usize)> = rounds.into_iter().map(|(x, y)| {
        match dir {
            Up => statics.iter().filter(|(sx, sy)| *sx == x && *sy < y).max().map(|(sx, sy)| (*sx, *sy + 1)).unwrap_or_else(|| (x, 0)),
            Left => statics.iter().filter(|(sx, sy)| *sy == y && *sx < x).max().map(|(sx, sy)| (*sx + 1, *sy)).unwrap_or_else(|| (0, y)),
            Down => statics.iter().filter(|(sx, sy)| *sx == x && *sy > y).min().map(|(sx, sy)| (*sx, *sy - 1)).unwrap_or_else(|| (x, ymax)),
            Right => statics.iter().filter(|(sx, sy)| *sy == y && *sx > x).min().map(|(sx, sy)| (*sx - 1, *sy)).unwrap_or_else(|| (xmax, y)),
        }
    }).collect();
    counts.into_iter().flat_map(|((x, y), count)| {
        let new_points: Vec<_> = match dir {
            Up => (0..count).map(|i| (x, y + i)).collect(),
            Left => (0..count).map(|i| (x + i, y)).collect(),
            Down => (0..count).map(|i| (x, y - i)).collect(),
            Right => (0..count).map(|i| (x - i, y)).collect(),
        };
        new_points.into_iter()
    }).collect()
}

fn find_repeat(mut rounds: Vec<(usize, usize)>, statics: &Vec<(usize, usize)>, xmax: usize, ymax: usize) -> (usize, usize, Vec<(usize, usize)>) {
    rounds.sort();
    let mut cache = HashMap::<Vec<(usize, usize)>, usize>::new();
    let mut i = 0;
    cache.insert(rounds.clone(), i);
    loop {
        i += 1;
        rounds = cycle(rounds, &statics, xmax, ymax);
        rounds.sort();
        match cache.get(&rounds) {
            Some(&prev) => {
                return (prev, i, rounds);
            }
            None => {
                cache.insert(rounds.clone(), i);
            }
        }
    }
}

aoc!(parse);

#[grid]
fn parse(gd: HashMap<(usize, usize), char>) -> (Vec<(usize, usize)>, Vec<(usize, usize)>, usize, usize) {
    let rounds: Vec<(usize, usize)> = gd.iter().filter(|(_point, c)| **c == 'O').map(|(point, _c)| *point).collect();
    let statics: Vec<(usize, usize)> = gd.iter().filter(|(_point, c)| **c == '#').map(|(point, _c)| *point).collect();
    (rounds, statics, gd.xmax(), gd.ymax())
}

const EXAMPLE: &str = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";