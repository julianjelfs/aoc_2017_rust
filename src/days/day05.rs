pub fn run(input: &str) {
    println!("Day 05, Part 1: {}", part1(input));
    println!("Day 05, Part 2: {}", part2(input));
}

fn register_updater_one(val: &mut i32) {
        *val += 1;
}

fn register_updater_two(val: &mut i32) {
    if *val >= 3 {
        *val -= 1;
    } else {
        *val += 1;
    }
}

fn part1(input: &str) -> usize {
    solve(input, register_updater_one)
}

fn part2(input: &str) -> usize {
    solve(input, register_updater_two)
}

fn solve<F>(input: &str, register_updater: F) -> usize
where F: Fn(&mut i32),
{
    let mut jumps: usize = 0;
    let mut index: i32 = 0;
    let mut register: Vec<i32> = input.lines().filter_map(|line| line.parse().ok()).collect();

    while (0..register.len() as i32).contains(&index) {
        let val = &mut register[index as usize];
        index += *val;
        register_updater(val);
        jumps += 1;
    }
    jumps
}
