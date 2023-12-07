pub use aoc_maker::*;
use regex::Regex;
use std::iter::Iterator;
use std::str::FromStr;
use std::vec::IntoIter;

#[macro_export]
macro_rules! aoc {
    (@common) => {
        fn get_input() -> Option<String> {
            let example = std::env::args().find(|arg| *arg == String::from("--example") || *arg == String::from("-e")).is_some();
            let day = std::path::Path::new(file!()).file_stem()?.to_str()?;
            let input_file = format!("./inputs/{day}.txt");
            let input = match example {
                true => String::from(EXAMPLE),
                false => std::fs::read_to_string(&input_file).ok()?,
            };
            Some(input.trim().replace("\r", "").to_string())
        }
    };
    () => {
        aoc!(@common);

        fn main() {
            let input = get_input().expect("Couldn't find input!");
            let first = first(&input);
            println!("First: {first}");
            let second = second(&input);
            println!("Second: {second}");
        }
    };
    (part1) => {
        aoc!(@common);

        fn main() {
            let input = get_input().expect("Couldn't find input!");
            let first = first(&input);
            println!("First: {first}");
        }
    };
    ($fun:ident) => {
        aoc!(@common);

        fn main() {
            let input = get_input().expect("Couldn't find input!");
            let first = first($fun(&input));
            println!("First: {first}");
            let second = second($fun(&input));
            println!("Second: {second}");
        }
    }
}

pub struct Numbers<T: FromStr + Copy = usize> {
    iter: IntoIter<T>,
    count: usize,
    numbers: Vec<T>,
}

impl<T: FromStr + Copy> Numbers<T> {
    pub fn parse(s: &String) -> Self {
        let re = Regex::new(r"\d+").expect("Weird regex");
        let numbers: Vec<T> = re.find_iter(&s).filter_map(|n| n.as_str().parse::<T>().ok()).collect();
        let count = numbers.len();
        let iter = numbers.to_vec().into_iter();
        Numbers {
            iter,
            count,
            numbers,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn vec(&self) -> Vec<T> {
        self.numbers.to_vec()
    }
}

impl<T: FromStr + Copy> Iterator for Numbers<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}