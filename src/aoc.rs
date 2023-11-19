mod d0; mod d1; mod d2; mod d3; mod d4; mod d5; mod d6; mod d7; mod d8; mod d9; mod d10; mod d11; mod d12; mod d13; mod d14; mod d15; mod d16; mod d17; mod d18; mod d19; mod d20; mod d21; mod d22; mod d23; mod d24; mod d25;

use crate::common::Res;
use std::path::Path;
use std::fs;
use std::ops::Fn;

macro_rules! prepare_day_functions {
    ($($module:tt),*) => {
        {
            let mut days = Vec::<Box<dyn Fn(String) -> (String, String)>>::new();
            $(
                let boxed = Box::new(|input: String| {
                    let first = $module::first($module::parse(input.to_owned()));
                    let second = $module::second($module::parse(input.to_owned()));
                    let unpack = |r: Res<$module::Output>| match r {
                        Ok(res) => res.to_string(),
                        Err(err) => err.to_string(),
                    };
                    return (unpack(first), unpack(second));
                });
                days.push(boxed);
            )*
            days
        }
    };
}

pub fn day(n: usize, input_directory: String) {
    let input_path = format!("{input_directory}/{n}.txt");
    let input = if Path::new(&input_path).exists() {
        str::replace(&fs::read_to_string(input_path).unwrap(), "\r\n", "\n")
    }
    else {
        // Logic to download from site ...
        panic!("uh oh");
    };
    
    let days = prepare_day_functions!(d0,d1,d2,d3,d4,d5,d6,d7,d8,d9,d10,d11,d12,d13,d14,d15,d16,d17,d18,d19,d20,d21,d22,d23,d24,d25);
    let (first, second) = days[n](input);
    
    println!("Day {n}");
    println!("First: {first}");
    println!("Second: {second}");
}