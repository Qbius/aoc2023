use aoc::*;

const EXAMPLE: &str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

fn first((seeds, mappings): (Vec<i64>, Vec<Vec<(i64, i64, i64)>>)) -> i64 {
    seeds.into_iter().map(|seed| mappings.iter().fold(seed, |val, segment_mappings| {
        match segment_mappings.iter().find(|(_, sorc_start, len)| val >= *sorc_start && (val - *sorc_start) < *len) {
            Some((dest_start, sorc_start, _)) => (dest_start - sorc_start) + val,
            None => val
        }
    })).min().expect("Input is empty")
}

fn second((seeds, mappings): (Vec<i64>, Vec<Vec<(i64, i64, i64)>>)) -> i64 {
    let new_seeds: Vec<i64> = seeds.chunks(2).flat_map(|chunk| chunk[0]..chunk[0] + chunk[1]).collect();
    first((new_seeds, mappings))
}

fn parse(input: &str) -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
    let segments: Vec<_> = input.split("\n\n").collect();
    let (first_line, rest_segments) = segments.split_first().expect("Something is wrong with the input");
    let seeds: Vec<_> = first_line[7..].split(' ').filter_map(|num| num.parse::<i64>().ok()).collect();

    (seeds, rest_segments.into_iter().map(|segment| {
        segment.split('\n').skip(1).map(|line| {
            let numbers: Vec<_> = line.split(' ').filter_map(|num| num.parse::<i64>().ok()).collect();
            let dest_start = numbers[0];
            let sorc_start = numbers[1];
            let len = numbers[2];
            (dest_start, sorc_start, len)
        }).collect()
    }).collect())
}

aoc!(parse);