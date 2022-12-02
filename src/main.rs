use std::fs;
use std::path::Path;
use std::process;
mod solutions;

fn main() {
    let day_string = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please enter the day number as the first argument!");
        process::exit(1)
    });

    let day = day_string.parse::<usize>().unwrap_or_else(|_| {
        println!("Please enter a positive day number!");
        process::exit(1)
    });

    if solutions::DAYS.len() <= day {
        println!("No solution found for Day #{}!", day);
        println!("Did you remember to register the function in the solutions module?");
        process::exit(1)
    }

    let input_file_str = format!("./input/day{}.txt", day);
    let input_file_path = Path::new(&input_file_str);

    if !input_file_path.is_file() {
        println!("No input file found for Day #{}!", day);
        println!("Did you remember to download the input file to the input folder?");
        process::exit(1)
    }

    println!("Running solution for Day #{}!", day);

    let input = fs::read_to_string(input_file_path).expect("file");
    let (part1, part2) = solutions::DAYS[day](&input);

    println!("Part 1: {}, Part 2: {}", part1, part2);
}
