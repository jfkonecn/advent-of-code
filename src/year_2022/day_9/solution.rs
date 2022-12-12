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

impl From<&str> for Command {
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
        // println!("{:?}", direction);
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
            // println!("{:?} - {:?}", head, rest);
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
        || (head.x == tail.x - 2 && head.y == tail.y - 2)
    {
        tail.x -= 1;
        tail.y -= 1;
    } else if (head.x == tail.x + 1 && head.y == tail.y + 2)
        || (head.x == tail.x + 2 && head.y == tail.y + 1)
        || (head.x == tail.x + 2 && head.y == tail.y + 2)
    {
        tail.x += 1;
        tail.y += 1;
    } else if (head.x == tail.x - 1 && head.y == tail.y + 2)
        || (head.x == tail.x - 2 && head.y == tail.y + 1)
        || (head.x == tail.x - 2 && head.y == tail.y + 2)
    {
        tail.x -= 1;
        tail.y += 1;
    } else if (head.x == tail.x + 1 && head.y == tail.y - 2)
        || (head.x == tail.x + 2 && head.y == tail.y - 1)
        || (head.x == tail.x + 2 && head.y == tail.y - 2)
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
    88,
    6337,
    solution_2,
    36,
    2455,
    "src",
    "year_2022",
    "day_9"
);

#[cfg(test)]
mod tests2 {
    use super::*;
    macro_rules! point_move_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (head, expected) = $value;
            let mut tail = Point { x: 0, y: 0 };
            move_tail(&head, &mut tail);
            assert_eq!(expected, tail);
        }
    )*
    }
}
    point_move_tests!(
        no_move_1:
        (
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 }
        ),
        no_move_2:
        (
            Point { x: 1, y: 1 },
            Point { x: 0, y: 0 }
        ),
        no_move_3:
        (
            Point { x: 1, y: 0 },
            Point { x: 0, y: 0 }
        ),
        no_move_4:
        (
            Point { x: 1, y: -1 },
            Point { x: 0, y: 0 }
        ),
        no_move_5:
        (
            Point { x: 0, y: -1 },
            Point { x: 0, y: 0 }
        ),
        no_move_6:
        (
            Point { x: -1, y: -1 },
            Point { x: 0, y: 0 }
        ),
        no_move_7:
        (
            Point { x: -1, y: 0 },
            Point { x: 0, y: 0 }
        ),
        no_move_8:
        (
            Point { x: -1, y: 1 },
            Point { x: 0, y: 0 }
        ),
        no_move_9:
        (
            Point { x: 0, y: 1 },
            Point { x: 0, y: 0 }
        ),
        up_1:
        (
            Point { x: 0, y: 2 },
            Point { x: 0, y: 1 }
        ),
        down_1:
        (
            Point { x: 0, y: -2 },
            Point { x: 0, y: -1 }
        ),
        right_1:
        (
            Point { x: 2, y: 0 },
            Point { x: 1, y: 0 }
        ),
        left_1:
        (
            Point { x: -2, y: 0 },
            Point { x: -1, y: 0 }
        ),
    );
}
