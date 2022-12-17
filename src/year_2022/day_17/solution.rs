use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::cmp::*;
use std::collections::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Command {
    PushLeft,
    PushRight,
}

impl From<char> for Command {
    fn from(c: char) -> Self {
        match c {
            '<' => Command::PushLeft,
            '>' => Command::PushRight,
            _ => panic!("unknown command!"),
        }
    }
}

fn parse_commands(file_contents: String) -> Vec<Command> {
    file_contents
        .chars()
        .filter(|x| !x.is_whitespace())
        .map_into()
        .collect_vec()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DashRock {
    points: Vec<(usize, usize)>,
}

impl DashRock {
    fn new(bottom_y: usize) -> Self {
        DashRock {
            points: vec![(2, bottom_y), (3, bottom_y), (4, bottom_y), (5, bottom_y)],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PlusRock {
    points: Vec<(usize, usize)>,
}

impl PlusRock {
    fn new(bottom_y: usize) -> Self {
        PlusRock {
            points: vec![
                (3, bottom_y + 2),
                (2, bottom_y + 1),
                (3, bottom_y + 1),
                (4, bottom_y + 1),
                (3, bottom_y),
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BackwardsLRock {
    points: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VerticalRock {
    points: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SquareRock {
    points: Vec<(usize, usize)>,
}

impl SquareRock {
    fn new(bottom_y: usize) -> Self {
        SquareRock {
            points: vec![
                (2, bottom_y + 1),
                (2, bottom_y),
                (3, bottom_y + 1),
                (3, bottom_y),
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Rock {
    Dash(DashRock),
    Plus(PlusRock),
    BackwardsL(BackwardsLRock),
    Vertical(VerticalRock),
    Square(SquareRock),
}

impl From<DashRock> for Rock {
    fn from(rock: DashRock) -> Self {
        Rock::Dash(rock)
    }
}

impl From<PlusRock> for Rock {
    fn from(rock: PlusRock) -> Self {
        Rock::Plus(rock)
    }
}

impl From<BackwardsLRock> for Rock {
    fn from(rock: BackwardsLRock) -> Self {
        Rock::BackwardsL(rock)
    }
}

impl From<VerticalRock> for Rock {
    fn from(rock: VerticalRock) -> Self {
        Rock::Vertical(rock)
    }
}

impl From<SquareRock> for Rock {
    fn from(rock: SquareRock) -> Self {
        Rock::Square(rock)
    }
}

struct Cave {
    falling_rock: Rock,
    fallen_rocks: Vec<Rock>,
    taken_space: HashSet<(usize, usize)>,
}

impl Cave {
    fn new() -> Self {
        Cave {
            falling_rock: DashRock::new(4).into(),
            fallen_rocks: vec![],
            taken_space: HashSet::new(),
        }
    }
}

pub fn solution_1(file_contents: String) -> usize {
    let commands = parse_commands(file_contents);
    println!("{:?}", commands);
    0
}

pub fn solution_2(file_contents: String) -> usize {
    let commands = parse_commands(file_contents);
    println!("{:?}", commands);
    0
}

challenge_test_suite!(
    solution_1,
    1,
    1,
    solution_2,
    1,
    1,
    "src",
    "year_2022",
    "day_17"
);
