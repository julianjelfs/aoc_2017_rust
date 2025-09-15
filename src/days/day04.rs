use std::collections::HashSet;

use itertools::Itertools;

pub fn run(input: &str) {
    println!("Day 04, Part 1: {}", part1(input));
    println!("Day 04, Part 2: {}", part2(input));
}

fn contains_anagrams(words: &[&str]) -> bool {
    let mut s = HashSet::new();
    for &word in words {
        let sorted: String = word.chars().sorted_unstable().collect();
        if !s.insert(sorted) {
            return true;
        }
    }
    false
}

fn contains_duplicates(words: &[&str]) -> bool {
    let mut set = HashSet::new();
    for &word in words {
        if !set.insert(word) {
            return true;
        }
    }
    false
}

fn solve<F>(input: &str, check: F) -> usize
where
    F: Fn(&[&str]) -> bool,
{
    input
        .lines()
        .filter(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            !check(&words)
        })
        .count()
}

fn part1(input: &str) -> usize {
    solve(input, contains_duplicates)
}

fn part2(input: &str) -> usize {
    solve(input, contains_anagrams)
}
