use std::env;
use std::fs;

mod days;

fn main() {
    // get the day number from command line
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <day>");
        std::process::exit(1);
    }

    let day: u32 = args[1].parse().expect("Day must be a number");
    let input_path = format!("inputs/day{:02}.txt", day);
    // let input_path = format!("inputs/day{:02}-test.txt", day);

    let input =
        fs::read_to_string(&input_path).unwrap_or_else(|_| panic!("Failed to read {}", input_path));

    match day {
        1 => days::day01::run(input.trim()),
        2 => days::day02::run(input.trim()),
        3 => days::day03::run(input.trim()),
        4 => days::day04::run(input.trim()),
        5 => days::day05::run(input.trim()),
        6 => days::day06::run(input.trim()),
        7 => days::day07::run(input.trim()),
        // keep adding as you go
        _ => eprintln!("Day {} not implemented yet", day),
    }
}
