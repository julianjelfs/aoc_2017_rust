pub fn run(input: &str) {
    println!("Day 01, Part 1: {}", part1(input));
    println!("Day 01, Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    // your solution here
    let _digits: Vec<u32>  = input.chars().map(|c| c.to_digit(10).expect("not a digit")).collect();
    println!("This is a the input: {input}");
    0
}

fn part2(_input: &str) -> i32 {
    // your solution here
    0
}
