use aoc::*;

const EXAMPLE: &str = "
Time:      7  15   30
Distance:  9  40  200";

fn first(info: Vec<(usize, usize)>) -> usize {
    info.into_iter().map(|(time, record)| (0..=time).filter(|held| (held * (time - held)) > record).count()).product()
}

fn second(info: Vec<(usize, usize)>) -> usize {
    let (times, records): (Vec<_>, Vec<_>) = info.into_iter().map(|(time, record)| (time.to_string(), record.to_string())).unzip();
    let time = times[..].join("").parse::<usize>().expect("Something is wrong with the input");
    let record = records[..].join("").parse::<usize>().expect("Something is wrong with the input");
    first(vec![(time, record)])
}

#[lines]
fn parse(lines: Vec<String>) -> Vec<(usize, usize)> {
    let [ref first, ref second] = lines[..] else { panic!("Input has more than two lines!") };
    first.split(' ').filter_map(|num| num.parse().ok()).zip(second.split(' ').filter_map(|num| num.parse().ok())).collect()
}

aoc!(parse);