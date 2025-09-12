pub fn run(input: &str) {
    println!("Day 01, Part 1: {}", part1(input));
    println!("Day 01, Part 2: {}", part2(input));
}

fn next_index(i: usize, l: usize) -> usize {
   (i + 1) % l
}

fn halfway(i: usize, l: usize) -> usize {
    (i + l / 2) % l
}

fn solve(input: &str, index_finder: fn(usize, usize) -> usize) -> u32 {
    let digits: Vec<u32>  = input.chars().map(|c| c.to_digit(10).expect("not a digit")).collect();
    let len = digits.len();
    let mut total = 0;

    for i in 0..len {
        if let (Some(a), Some(b)) = (digits.get(i), digits.get(index_finder(i, len))) {
            if a == b {
                total += a;
            }
        }
    }
    total
}

fn part1(input: &str) -> u32 {
    solve(input, next_index)
}

fn part2(input: &str) -> u32 {
    solve(input, halfway)
}
