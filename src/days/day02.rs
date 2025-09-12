pub fn run(input: &str) {
    println!("Day 01, Part 1: {}", part1(input));
    println!("Day 01, Part 2: {}", part2(input));
}

fn minmax(digits: &Vec<u32>) -> u32 {
    let max = digits.iter().max().unwrap();
    let min = digits.iter().min().unwrap();
    max - min
}

fn divisible(digits: &Vec<u32>) -> u32 {
    let mut result = 0;
    for x1 in digits.iter() {
        for y1 in digits.iter() {
            if x1 != y1 {
                if x1 % y1 == 0 {
                    result += x1 / y1;
                }
            }
        }
    }
    result
}

fn solve<F>(input: &str, logic: F) -> u32
where
    F: Fn(&Vec<u32>) -> u32,
{
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0;

    for line in lines {
        let digits: Vec<u32> = line.split_whitespace().filter_map(|word| word.parse().ok()).collect();
        total += logic(&digits);
    }
    total
}

fn part2(input: &str) -> u32 {
    solve(input, divisible)
}

fn part1(input: &str) -> u32 {
    solve(input, minmax)
}
