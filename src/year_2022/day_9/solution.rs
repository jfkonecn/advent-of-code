use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::collections::*;

#[derive(Debug, Clone)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}
#[derive(Debug, Clone)]
struct Command {
    amount: isize,
    direction: Direction,
}

impl<'a> From<&str> for Command {
    fn from(str: &str) -> Self {
        let str = str.to_owned();
        let mut raw_strs = str.split_ascii_whitespace();
        let cmd_str = raw_strs.next().unwrap();
        let amount = raw_strs.next().unwrap().parse().unwrap();

        let direction = match cmd_str {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unreachable!("Unknown command {}", cmd_str),
        };
        Command { amount, direction }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Point {
    x: isize,
    y: isize,
}

fn parse_commands(file_contents: String) -> Vec<Command> {
    file_contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .map_into()
        .collect_vec()
}

fn simulate(commands: Vec<Command>) -> Vec<Point> {
    let mut visited = HashSet::new();
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    visited.insert(tail.clone());

    for Command { amount, direction } in commands {
        for _ in 0..amount {
            match direction {
                Direction::Up => {
                    head.y += 1;
                }
                Direction::Down => {
                    head.y -= 1;
                }
                Direction::Left => {
                    head.x -= 1;
                }
                Direction::Right => {
                    head.x += 1;
                }
            };
            if head.y == tail.y && head.x > tail.x + 1 {
                tail.x += 1;
            } else if head.y == tail.y && head.x < tail.x - 1 {
                tail.x -= 1;
            } else if head.x == tail.x && head.y > tail.y + 1 {
                tail.y += 1;
            } else if head.x == tail.x && head.y < tail.y - 1 {
                tail.y -= 1;
            } else if (head.x == tail.x - 1 && head.y == tail.y - 2)
                || (head.x == tail.x - 2 && head.y == tail.y - 1)
            {
                tail.x -= 1;
                tail.y -= 1;
            } else if (head.x == tail.x + 1 && head.y == tail.y + 2)
                || (head.x == tail.x + 2 && head.y == tail.y + 1)
            {
                tail.x += 1;
                tail.y += 1;
            } else if (head.x == tail.x - 1 && head.y == tail.y + 2)
                || (head.x == tail.x - 2 && head.y == tail.y + 1)
            {
                tail.x -= 1;
                tail.y += 1;
            } else if (head.x == tail.x + 1 && head.y == tail.y - 2)
                || (head.x == tail.x + 2 && head.y == tail.y - 1)
            {
                tail.x += 1;
                tail.y -= 1;
            }
            visited.insert(tail.clone());
        }
    }

    Vec::from_iter(visited)
}

pub fn solution_1(file_contents: String) -> usize {
    let commands = parse_commands(file_contents);
    let vec = simulate(commands);
    vec.len()
}

pub fn solution_2(file_contents: String) -> usize {
    1
}

challenge_test_suite!(
    solution_1,
    13,
    6337,
    solution_2,
    1,
    1,
    "src",
    "year_2022",
    "day_9"
);
