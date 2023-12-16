use std::collections::HashMap;
use aoc::*;

struct Beam {
    point: (usize, usize),
    dir: Direction,
    
}

#[derive(Eq, PartialEq, Debug)]
enum BeamState {
    Continue,
    Stop,
}
use BeamState::*;

impl Beam {
    fn new(point: (usize, usize), dir: Direction) -> Self {
        Beam {
            point,
            dir,
        }
    }

    fn default() -> Self {
        Beam::new((0, 0), Right)
    }

    fn step(&mut self, gd: &HashMap<(usize, usize), char>, history: &mut HashMap<(usize, usize), Vec<Direction>>) -> (BeamState, Option<Beam>) {
        if history.get(&self.point).map(|dirvec| dirvec.contains(&self.dir)).unwrap_or(false) {
            return (Stop, None);
        }
        history.entry(self.point.clone()).or_default().push(self.dir);
        let c = gd.get(&self.point).expect("went too far");

        let (new_dir, spawned) = match c {
            '|' => match self.dir {
                Right => (Up, Some(Beam::new(self.point.clone(), Down))),
                Left => (Up, Some(Beam::new(self.point.clone(), Down))),
                dir => (dir, None),
            },
            '-' => match self.dir {
                Up => (Left, Some(Beam::new(self.point.clone(), Right))),
                Down => (Left, Some(Beam::new(self.point.clone(), Right))),
                dir => (dir, None),
            },
            '/' => match self.dir {
                Up => (Right, None),
                Right => (Up, None),
                Left => (Down, None),
                Down => (Left, None),
            },
            '\\' => match self.dir {
                Up => (Left, None),
                Right => (Down, None),
                Left => (Up, None),
                Down => (Right, None),
            },
            '.' => (self.dir, None),
            _ => panic!("weird char"),
        };
        self.dir = new_dir;
        let (x, y) = self.point;
        let will_continue = match self.dir {
            Up => y != 0,
            Left => x != 0,
            Right => x != gd.xmax(),
            Down => y != gd.ymax(),
        };
        if will_continue {
            self.point = self.dir.traverse(self.point);
            (Continue, spawned)
        }
        else {
            (Stop, spawned)
        }
    }
}

fn trek(gd: &HashMap<(usize, usize), char>, start_beam: Beam) -> usize {
    let mut beams: Vec<Beam> = vec![start_beam];
    let mut history = HashMap::<(usize, usize), Vec<Direction>>::new();
    while !beams.is_empty() {
        match beams.pop() {
            Some(mut beam) => {
                loop {
                    let (state, spawned) = beam.step(gd, &mut history);
                    if let Some(new_beam) = spawned {
                        beams.push(new_beam);
                    }
                    if let Stop = state {
                        break;
                    }
                }
            }
            None => {
                ()
            }
        }
    }
    history.len()
}

#[grid]
fn first(gd: HashMap<(usize, usize), char>) -> usize {
    trek(&gd, Beam::default())
}

#[grid]
fn second(gd: HashMap<(usize, usize), char>) -> usize {
    let xmax = gd.xmax();
    let ymax = gd.ymax();
    let top = (0..=xmax).map(|x| ((x, 0), Down));
    let right = (0..=ymax).map(|y| ((xmax, y), Left));
    let bottom = (0..=xmax).map(|x| ((x, ymax), Up));
    let left = (0..=ymax).map(|y| ((0, y), Right));
    top.chain(right).chain(bottom).chain(left).map(|(point, dir)| Beam::new(point, dir)).map(|beam| trek(&gd, beam)).max().unwrap()
}

aoc!();

const EXAMPLE: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";