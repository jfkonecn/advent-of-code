use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::{borrow::Borrow, collections::*};

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

fn simulate(commands: Vec<Command>, rope_size: usize) -> Vec<Point> {
    let mut visited = HashSet::new();
    let mut head = Point { x: 0, y: 0 };

    let mut rest = (1..rope_size).map(|_| Point { x: 0, y: 0 }).collect_vec();
    visited.insert(head.clone());

    for Command { amount, direction } in commands {
        println!("{:?}", direction);
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
            let mut prev = &head;
            for mut tail in rest.iter_mut() {
                move_tail(prev, &mut tail);
                prev = tail;
            }
            visited.insert(prev.clone());
            println!("{:?} - {:?}", head, rest);
        }
    }

    Vec::from_iter(visited)
}

fn move_tail(head: &Point, tail: &mut Point) {
    // println!("({:?}, {:?})", head, tail);
    if head.y == tail.y && head.x == tail.x + 2 {
        tail.x += 1;
    } else if head.y == tail.y && head.x == tail.x - 2 {
        tail.x -= 1;
    } else if head.x == tail.x && head.y == tail.y + 2 {
        tail.y += 1;
    } else if head.x == tail.x && head.y == tail.y - 2 {
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
}

pub fn solution_1(file_contents: String) -> usize {
    let commands = parse_commands(file_contents);
    let vec = simulate(commands, 2);
    vec.len()
}

pub fn solution_2(file_contents: String) -> usize {
    let commands = parse_commands(file_contents);
    let vec = simulate(commands, 10);
    vec.len()
}

challenge_test_suite!(
    solution_1,
    13,
    6337,
    solution_2,
    1,
    // 16 is wrong
    // 13 is wrong
    // 1 is wrong
    1,
    "src",
    "year_2022",
    "day_9"
);
