use std::collections::HashMap;

pub fn run(input: &str) {
    println!("Day 01, Part 1: {}", part1(input));
    println!("Day 01, Part 2: {}", part2(input));
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn neighbour_coordinates((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        ((x + 1), y),
        (x + 1, y - 1),
        (x, y - 1),
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

fn sum_of_neighbours(coord: (i32, i32), values: &HashMap<(i32, i32), u32>) -> u32 {
    neighbour_coordinates(coord)
        .iter()
        .filter_map(|c| values.get(c))
        .sum()
}

fn part1(input: &str) -> u32 {
    solve(input, false)
}

fn part2(input: &str) -> u32 {
    solve(input, true)
}

fn solve(input: &str, check_neighbours: bool) -> u32 {
    let steps: u32 = input.parse().ok().unwrap();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut values: HashMap<(i32, i32), u32> = HashMap::new();
    values.insert((0, 0), 1);

    let mut left = 0;
    let mut right = 0;
    let mut top = 0;
    let mut bottom = 0;

    let mut direction = Direction::Right;

    for _ in 0..steps - 1 {
        match direction {
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
            Direction::Up => y -= 1,
        }

        direction = match direction {
            Direction::Down if y > bottom => {
                bottom += 1;
                Direction::Right
            }
            Direction::Left if x < left => {
                left -= 1;
                Direction::Down
            }
            Direction::Right if x > right => {
                right += 1;
                Direction::Up
            }
            Direction::Up if y < top => {
                top -= 1;
                Direction::Left
            }
            _ => direction,
        };

        if check_neighbours {
            let sum = sum_of_neighbours((x, y), &values);
            if sum > steps {
                return sum;
            } else {
                values.insert((x, y), sum);
            }
        }
    }

    (x.abs() + y.abs()) as u32
}
