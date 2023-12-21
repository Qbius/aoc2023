use std::collections::{HashMap, HashSet};

use aoc::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct TimePoint<const LIMIT: usize>(usize, usize, usize);

impl<const LIMIT: usize> TimePoint<LIMIT> {
    fn get_neighbours(&self) -> Vec<TimePoint<LIMIT>> {
        let TimePoint(x, y, t) = &self;
        let mut nghbrs: Vec<TimePoint<LIMIT>> = if *t < LIMIT {
            vec![TimePoint(*x + 1, *y, *t + 1), TimePoint(*x, *y + 1, *t + 1)]
        }
        else {
            return Vec::new();
        };
        if *x > 0 {
            nghbrs.push(TimePoint(*x - 1, *y, *t + 1));
        }
        if *y > 0 {
            nghbrs.push(TimePoint(*x, *y - 1, *t + 1));
        }
        nghbrs
    }
}

#[grid]
fn first(gd: HashMap<(usize, usize), char>) -> usize {
    let (sx, sy) = gd.iter().find_map(|(point, c)| match *c == 'S' {true => Some(point), false => None}).expect("wtf").clone();
    let mut good_steps: HashSet<_> = gd.points('.').into_iter().collect();
    good_steps.insert((sx, sy));
    djikstra::<64>((sx, sy), &good_steps)
}

fn djikstra<const LIMIT: usize>((sx, sy): (usize, usize), good_steps: &HashSet<(usize, usize)>) -> usize {
    let mut distances: HashMap<TimePoint<LIMIT>, usize> = HashMap::new();
    distances.insert(TimePoint(sx, sy, 0), 0);
    let mut visited: HashSet::<TimePoint<LIMIT>> = HashSet::new();
    loop {
        let Some((distance, current)) = distances.iter().filter_map(|(point, distance)| match visited.contains(point) {true => None, false => Some((*distance, point))}).min() else {
            return visited.into_iter().filter(|TimePoint::<LIMIT>(_, _, t)| *t == LIMIT).count();
        };

        visited.insert(current.clone());
        let ngbhrs = current.get_neighbours().into_iter().filter(|TimePoint::<LIMIT>(x, y, _)| good_steps.contains(&(*x, *y))).collect_vec();
        for nghbr in ngbhrs.iter() {
            distances.insert(nghbr.clone(), distance + 1);
        }
    }
}

aoc!(part1);
 
const EXAMPLE: &str = "
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";