use std::collections::HashSet;

pub fn run(input: &str) {
    let memory_bank = get_memory_bank(input);
    let (part1, memory_bank) = solve(memory_bank);
    let (part2, _) = solve(memory_bank);
    println!("Day 06, Part 1: {}", part1);
    println!("Day 06, Part 2: {}", part2);
}

fn get_memory_bank(input: &str) -> Vec<u32> {
    if let Some(line) = input.lines().next() {
        line.split_whitespace()
            .filter_map(|w| w.parse().ok())
            .collect::<Vec<u32>>()
    } else {
        panic!("Unable to parse memory bank")
    }
}

fn max(memory_bank: &[u32]) -> (usize, u32) {
    memory_bank.iter().enumerate().fold(
        (0, 0),
        |(max_i, max_v), (i, &v)| {
            if v > max_v {
                (i, v)
            } else {
                (max_i, max_v)
            }
        },
    )
}

fn balance(memory_bank: &mut [u32]) {
    let len = memory_bank.len();
    let (index, max) = max(memory_bank);
    memory_bank[index] = 0;
    for i in 0..max as usize {
        memory_bank[(index + i + 1) % len] += 1;
    }
}

fn solve(mut memory_bank: Vec<u32>) -> (usize, Vec<u32>) {
    let mut arrangements: HashSet<Vec<u32>> = HashSet::new();
    let mut cycles = 0;
    arrangements.insert(memory_bank.clone());

    loop {
        balance(&mut memory_bank);
        cycles += 1;
        if !arrangements.insert(memory_bank.clone()) {
            break;
        }
    }
    (cycles, memory_bank)
}
