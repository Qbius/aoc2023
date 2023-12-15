use std::collections::HashMap;

use aoc::*;

#[derive(Debug)]
enum Dir {
    Row,
    Column,
}

fn find_mirror(lns: Vec<String>) -> Option<(usize, usize)> {
    let first_line = lns.first()?.clone();
    println!("{first_line}");
    lns[1..].iter().enumerate().rev().find_map(|(mirror_i_minus_one, rev_str)| {
        let mirror_i = mirror_i_minus_one + 1;
        let reflection_length = 1 + mirror_i / 2;
        if first_line == *rev_str {
            println!("{mirror_i} {}", *rev_str);
            match (1..=reflection_length).all(|i| lns[i] == lns[mirror_i - i]) {
                true => Some((mirror_i, reflection_length)),
                false => None,
            }
        }
        else {
            None
        }
    })
}

fn first(rowsandcolumns: Vec<(Vec<String>, Vec<String>)>) -> usize {
    rowsandcolumns.into_iter().flat_map(|(rows, cols)| {
        let row_count = rows.len();
        let col_count = cols.len();
        let reverse_rows: Vec<String> = rows.iter().cloned().rev().collect();
        let reverse_cols: Vec<String> = cols.iter().cloned().rev().collect();
        //let row_mirror = find_mirror(rows).map(|(_, len)| (0, len)).or_else(|| find_mirror(reverse_rows).map(|(i, len)| (row_count - i - 1, len))).map(|i| (Dir::Row, i));
        //let col_mirror = find_mirror(cols).map(|(_, len)| (0, len)).or_else(|| find_mirror(reverse_cols).map(|(i, len)| (col_count - i - 1, len))).map(|i| (Dir::Column, i));
        vec![
            find_mirror(rows).map(|(_, len)| (0, len)).map(|i| (Dir::Row, i)).or(find_mirror(reverse_rows).map(|(i, len)| (row_count - i - 1, len)).map(|i| (Dir::Row, i))),
            find_mirror(cols).map(|(_, len)| (0, len)).map(|i| (Dir::Column, i)).or(find_mirror(reverse_cols).map(|(i, len)| (col_count - i - 1, len)).map(|i| (Dir::Column, i))),
        ].into_iter().filter_map(std::convert::identity)
        // let res = row_mirror.or(col_mirror);
        // res
    }).map(|(dir, (start_i, len))| match dir {
        Dir::Row => 100,
        Dir::Column => 1,
    } * (start_i + len)).sum()

}

fn second(rowsandcolumns: Vec<(Vec<String>, Vec<String>)>) -> i32 {
    0
}

fn parse(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input.split("\n\n").filter_map(|s| {
        let rows: Vec<String> = s.trim().split('\n').map(String::from).collect();
        let width = rows.first()?.len();
        let columns: Vec<String> = (0..width).map(|x| {
            let column_chunks: Vec<String> = rows.iter().map(|row| row[x..x + 1].to_string()).collect();
            column_chunks[..].join("")
        }).collect();
        Some((rows, columns))
    }).collect()
}

aoc!(parse);

const EXAMPLE: &str = "
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
#...##..#";

const _EXAMPLE: &str = "
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