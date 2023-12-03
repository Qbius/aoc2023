pub use aoc_maker::*;

#[macro_export]
macro_rules! aoc {
    (@common) => {
        fn get_input() -> Option<String> {
            let example = std::env::args().find(|arg| *arg == String::from("--example") || *arg == String::from("-e")).is_some();
            let day = std::path::Path::new(file!()).file_stem()?.to_str()?;
            let input_file = format!("./inputs/{day}.txt");
            Some(match example {
                true => String::from(EXAMPLE),
                false => std::fs::read_to_string(&input_file).ok()?,
            })
        }
    };
    () => {
        aoc!(@common);

        fn main() {
            let input = get_input().expect("Couldn't find input!");
            let first = first(input.trim());
            let second = second(input.trim());
            println!("First: {first}");
            println!("Second: {second}");
        }
    };
    (part1) => {
        aoc!(@common);

        fn main() {
            let input = get_input().expect("Couldn't find input!");
            let first = first(input.trim());
            println!("First: {first}");
        }
    };
}