pub fn run(input: &str) {
    println!("Day 01, Part 1: {}", part1(input));
    println!("Day 01, Part 2: {}", part2(input));
}

fn solve<F>(input: &str, index_finder: F) -> u32
where
    F: Fn(usize, usize) -> usize
{
    let digits: Vec<u32>  = input.chars().map(|c| c.to_digit(10).expect("not a digit")).collect();
    let len = digits.len();
    digits.iter().enumerate().filter(|(i, &d)| d == digits[index_finder(*i, len)]).map(|(_, d)| d).sum()
}

fn part1(input: &str) -> u32 {
    solve(input, |i, l| (i + 1) % l)
}

fn part2(input: &str) -> u32 {
    solve(input, |i, l| (i + l / 2) % l)
}
