use aoc::*;

#[lines]
fn first(ls: Vec<String>) -> usize {
    let vertices = ls.into_iter().fold(vec![(0isize, 0isize)], |mut verts, line| match line.split(' ').collect::<Vec<_>>()[..] {
        [dir_str, magnitude_str, _] => {
            let dir = Direction::from_char(dir_str.chars().nth(0).expect("weird dir"));
            let magnitude = magnitude_str.parse::<isize>().expect("weird magnitude");
            let last_point = verts.last().unwrap().clone();
            let vertex = dir.itraverse_n(last_point, magnitude);
            verts.push(vertex);
            verts
        }
        _ => {
            panic!("weird input");
        }
    });
    iarea(vertices)
}

#[lines]
fn second(ls: Vec<String>) -> usize {
    let vertices = ls.into_iter().fold(vec![(0isize, 0isize)], |mut verts, line| match line.split(' ').collect::<Vec<_>>()[..] {
        [_, _, color_str] => {
            let dir = Direction::from_char(['R', 'D', 'L', 'U'][color_str.chars().nth(7).and_then(|c| c.to_digit(10)).unwrap() as usize]);
            let magnitude = isize::from_str_radix(&color_str[2..7], 16).unwrap();
            let last_point = verts.last().unwrap().clone();
            let vertex = dir.itraverse_n(last_point, magnitude);
            verts.push(vertex);
            verts
        }
        _ => {
            panic!("weird input");
        }
    });
    iarea(vertices)
}

aoc!();

const EXAMPLE: &str = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";