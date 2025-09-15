use std::collections::HashSet;

pub fn run(input: &str) {
    let (cycles, memory_bank) = part1(input);
    println!("Day 06, Part 1: {}", cycles);
    println!("Day 06, Part 2: {}", part2(memory_bank));
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
    let (index, max) = max(memory_bank);
    memory_bank[index] = 0;
    for i in 0..max as usize {
        memory_bank[(index + i + 1) % 16] += 1;
    }
}

fn part1(input: &str) -> (usize, Vec<u32>) {
    let mut arrangements: HashSet<Vec<u32>> = HashSet::new();
    let mut memory_bank = get_memory_bank(input);
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

fn part2(mut memory_bank: Vec<u32>) -> usize {
    let mut arrangements: HashSet<Vec<u32>> = HashSet::new();
    let mut cycles = 0;
    arrangements.insert(memory_bank.clone());

    loop {
        balance(&mut memory_bank);
        cycles += 1;
        let clone = memory_bank.clone();
        if !arrangements.insert(clone) {
            break;
        }
    }
    cycles
}
