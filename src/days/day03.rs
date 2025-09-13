pub fn run(input: &str) {
    println!("Day 01, Part 1: {}", part1(input));
    println!("Day 01, Part 2: {}", part2(input));
}

fn part2(_input: &str) -> u32 {
    0
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(input: &str) -> u32 {
    let steps: u32 = input.parse().ok().unwrap();
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut left = 0;
    let mut right = 0;
    let mut top = 0;
    let mut bottom = 0;

    let mut direction = Direction::Right;

    for _ in 0..steps-1 {
        match direction {
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
            Direction::Up => y -= 1,
        }

        direction = match direction {
            Direction::Down if y > bottom => { bottom += 1; Direction::Right },
            Direction::Left if x < left => { left -= 1; Direction::Down },
            Direction::Right if x > right => { right += 1; Direction::Up },
            Direction::Up if y < top => { top -= 1; Direction::Left },
            _ => direction,
        };
    }

    (x.abs() + y.abs()) as u32
}
