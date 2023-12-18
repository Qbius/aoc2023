use std::collections::{HashMap, HashSet};

use aoc::*;

fn djikstra(start: (usize, usize), end: (usize, usize), gd: &HashMap<(usize, usize), usize>) -> Option<usize> {
    let mut distances: HashMap<(usize, usize), ((Direction, usize), usize)> = gd.keys().cloned().filter(|point| *point != start).map(|point| (point, ((Left, 0), usize::MAX))).collect();
    distances.insert(start.clone(), ((Left, 0usize), 0));
    let mut not_visisted: HashSet::<(usize, usize)> = gd.keys().cloned().into_iter().collect();
    loop {
        let Some(&current) = not_visisted.iter().min_by_key(|&point| distances.get(point).map(|(_, distance)| *distance).unwrap_or(usize::MAX)) else {
            return None;
        };
        not_visisted.remove(&current);
        let Some(&((dir, dircount), distance)) = distances.get(&current) else {
            return None;
        };
        let (cx, cy) = current;
        let mut possible_neighbours = vec![((cx + 1, cy), Right), ((cx, cy + 1), Down)];
        if cx > 0 {
            possible_neighbours.push(((cx - 1, cy), Left));
        }
        if cy > 0 {
            possible_neighbours.push(((cx, cy - 1), Up));
        }
        possible_neighbours.into_iter().filter(|(point, _)| not_visisted.contains(point)).filter(|(_, ndir)| ndir.mirror() != dir).filter(|(_, ndir)| dircount < 3 || *ndir != dir).filter_map(|(point, ndir)| gd.get(&point).map(|&d| (point, ndir, d))).for_each(|(npoint, ndir, d)| {
            let dirinfo = match ndir == dir {
                true => (ndir, dircount + 1),
                false => (ndir, 1),
            };
            distances.insert(npoint, (dirinfo, distance + d));
        });

        if !not_visisted.contains(&end) {
            let mut x = distances.iter().collect::<Vec<_>>();
            x.sort();
            println!("{:?}", x);
            return distances.get(&end).map(|(_, distance)| *distance);
        }
    }
}

#[grid]
fn first(gd: HashMap<(usize, usize), char>) -> usize {
    let usize_gd: HashMap<(usize, usize), usize> = gd.into_iter().map(|(point, c)| (point, c.to_digit(10).expect("wtf") as usize)).collect();
    djikstra((0, 0), (usize_gd.xmax(), usize_gd.ymax()), &usize_gd).expect("no path found")
}

aoc!(part1);

const EXAMPLE: &str = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";