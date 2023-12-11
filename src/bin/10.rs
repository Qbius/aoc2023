use std::collections::HashMap;
use std::iter::Iterator;
type Grid = HashMap<(usize, usize), char>;

use aoc::*;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}
use Direction::*;

impl Direction {
    fn mirror(&self) -> Self {
        match self {
            Left => Right,
            Up => Down,
            Right => Left,
            Down => Up,
        }
    }

    fn traverse(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Left => (x - 1, y),
            Up => (x, y - 1),
            Right => (x + 1, y),
            Down => (x, y + 1),
        }
    }
}

struct Loop {
    current: char,
    prev: Direction,
    point: (usize, usize),
    gd: Grid,
}

impl Loop {
    fn new(gd: Grid) -> Self {
        let point = gd.iter().find_map(|((x, y), c)| match c {'S' => Some((x.clone(), y.clone())), _ => None}).expect("No starting point!");
        let current = 'J';
        let prev = Left;
        Loop {
            current,
            prev,
            point,
            gd,
        }
    }
}

impl Iterator for Loop {
    type Item = ((usize, usize), char);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            'S' => {
                None
            }
            _ => {           
                let dir = match (self.current, self.prev) {
                    ('|', Up) => Down,
                    ('|', Down) => Up,
                    ('-', Left) => Right,
                    ('-', Right) => Left,
                    ('L', Up) => Right,
                    ('L', Right) => Up,
                    ('J', Up) => Left,
                    ('J', Left) => Up,
                    ('7', Left) => Down,
                    ('7', Down) => Left,
                    ('F', Right) => Down,
                    ('F', Down) => Right,
                    _ => panic!("esoteric pipe"),
                };
                self.point = dir.traverse(self.point);
                self.current = self.gd.get(&self.point).expect("went off the grid").clone();
                self.prev = dir.mirror();
                Some((self.point, self.current))
            }
        }
    }
}

#[grid]
fn first(gd: Grid) -> usize {
    (Loop::new(gd).count() as f64 / 2f64).ceil() as usize
}

#[grid]
fn second(gd: Grid) -> usize {
    let mut rows = HashMap::<usize, Vec<(usize, char)>>::new();
    Loop::new(gd).for_each(|((x, y), c)| rows.entry(y).or_default().push((x, c)));
    rows.into_iter().filter_map(|(_, xs2cs)| {
        let as_map: HashMap<_, _> = xs2cs.into_iter().collect();
        let start = as_map.keys().min()?.clone();
        let end = as_map.keys().max()?.clone();
        let (sum, false, _) = (start..=end).fold((0, false, None), |(acc, in_loop, prevchar), i| {
            let ch = as_map.get(&i);
            let res = match ch {
                Some('|') => (acc, !in_loop, prevchar),
                Some('F') => (acc, in_loop, Some('F')),
                Some('L') => (acc, in_loop, Some('L')),
                Some('S') => match prevchar {Some('F') => (acc, !in_loop, None), Some('L') => (acc, in_loop, None), _ => panic!("strangle loop")},
                Some('J') => match prevchar {Some('F') => (acc, !in_loop, None), Some('L') => (acc, in_loop, None), _ => panic!("strangle loop")},
                Some('7') => match prevchar {Some('F') => (acc, in_loop, None), Some('L') => (acc, !in_loop, None), _ => panic!("strangle loop")},
                Some('-') => (acc, in_loop, prevchar),
                _ => (acc + match in_loop {true => 1, false => 0}, in_loop, prevchar),
            };
            res
        }) else { panic!("strange loop") };
        Some(sum)
    }).sum()
}

const EXAMPLE: &str = "
FF7F7F7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LSLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

aoc!();