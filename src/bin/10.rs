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
    type Item = (usize, usize);

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
                Some(self.point)
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
    let lp: Vec<(usize, usize)> = Loop::new(gd).collect();
    println!("{lp:?}");
    0
}

const EXAMPLE: &str = "
-L|F7
7F-7|
L|7||
-L-S|
L|-JF";

aoc!();