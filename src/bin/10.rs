use std::collections::HashMap;
use std::iter::Iterator;
type Grid = HashMap<(usize, usize), char>;

use aoc::*;

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
    let vertices: Vec<(usize, usize)> = Loop::new(gd.clone()).filter_map(|(point, c)| match c {'S' | 'J' | 'L' | '7' | 'F' => Some(point), _ => None}).collect();
    area(vertices) - Loop::new(gd).count()
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