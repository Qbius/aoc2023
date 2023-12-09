use aoc::*;

fn first(rows_of_rows: Vec<Vec<Vec<i32>>>) -> i32 {
    rows_of_rows.into_iter().flat_map(|rows| rows.into_iter().filter_map(|row| row.last().cloned())).sum()
}

fn second(rows_of_rows: Vec<Vec<Vec<i32>>>) -> i32 {
    rows_of_rows.into_iter().map(|rows| rows.into_iter().rev().fold(0, |acc, row| row.first().expect("idk") - acc)).sum()
}

#[lines]
fn parse(ls: Vec<String>) -> Vec<Vec<Vec<i32>>> {
    ls.into_iter().map(|line| {
        let numbers: Vec<i32> = line.split(' ').filter_map(|numstr| numstr.parse::<i32>().ok()).collect();
        create_rows(numbers)
    }).collect()
}

fn create_rows(numbers: Vec<i32>) -> Vec<Vec<i32>> {
    let mut rows = vec![numbers];
    loop {
        let last = rows.last().expect("idk");
        let differences: Vec<_> = last[0..last.len() - 1].iter().cloned().zip(last[1..last.len()].iter().cloned()).map(|(a, b)| b - a).collect();
        if differences.iter().all(|&diff| diff == 0) {
            return rows;
        }
        else {
            rows.push(differences);
        }
    }
}

aoc!(parse);

const EXAMPLE: &str = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";