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

trait Rock {
    fn next_rock(&self, bottom_y: usize) -> Box<dyn Rock>;
    fn get_points(&self) -> &Vec<(usize, usize)>;
    fn set_points(&mut self, vec: Vec<(usize, usize)>);
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

impl Rock for DashRock {
    fn next_rock(&self, bottom_y: usize) -> Box<dyn Rock> {
        Box::new(PlusRock::new(bottom_y))
    }

    fn get_points(&self) -> &Vec<(usize, usize)> {
        &self.points
    }

    fn set_points(&mut self, vec: Vec<(usize, usize)>) {
        self.points = vec;
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

impl Rock for PlusRock {
    fn next_rock(&self, bottom_y: usize) -> Box<dyn Rock> {
        Box::new(BackwardsLRock::new(bottom_y))
    }

    fn get_points(&self) -> &Vec<(usize, usize)> {
        &self.points
    }

    fn set_points(&mut self, vec: Vec<(usize, usize)>) {
        self.points = vec;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BackwardsLRock {
    points: Vec<(usize, usize)>,
}

impl BackwardsLRock {
    fn new(bottom_y: usize) -> Self {
        BackwardsLRock {
            points: vec![
                (2, bottom_y),
                (3, bottom_y),
                (4, bottom_y),
                (4, bottom_y + 1),
                (4, bottom_y + 2),
            ],
        }
    }
}

impl Rock for BackwardsLRock {
    fn next_rock(&self, bottom_y: usize) -> Box<dyn Rock> {
        Box::new(VerticalRock::new(bottom_y))
    }

    fn get_points(&self) -> &Vec<(usize, usize)> {
        &self.points
    }

    fn set_points(&mut self, vec: Vec<(usize, usize)>) {
        self.points = vec;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VerticalRock {
    points: Vec<(usize, usize)>,
}

impl VerticalRock {
    fn new(bottom_y: usize) -> Self {
        VerticalRock {
            points: vec![
                (2, bottom_y),
                (2, bottom_y + 1),
                (2, bottom_y + 2),
                (3, bottom_y + 3),
            ],
        }
    }
}

impl Rock for VerticalRock {
    fn next_rock(&self, bottom_y: usize) -> Box<dyn Rock> {
        Box::new(SquareRock::new(bottom_y))
    }

    fn get_points(&self) -> &Vec<(usize, usize)> {
        &self.points
    }

    fn set_points(&mut self, vec: Vec<(usize, usize)>) {
        self.points = vec;
    }
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

impl Rock for SquareRock {
    fn next_rock(&self, bottom_y: usize) -> Box<dyn Rock> {
        Box::new(DashRock::new(bottom_y))
    }

    fn get_points(&self) -> &Vec<(usize, usize)> {
        &self.points
    }

    fn set_points(&mut self, vec: Vec<(usize, usize)>) {
        self.points = vec;
    }
}

struct Cave {
    falling_rock: Box<dyn Rock>,
    fallen_rocks: Vec<Box<dyn Rock>>,
    taken_space: HashSet<(usize, usize)>,
}

impl Cave {
    fn new() -> Self {
        Cave {
            falling_rock: Box::new(DashRock::new(4)),
            fallen_rocks: vec![],
            taken_space: HashSet::new(),
        }
    }

    fn intersects(&self, vec: &Vec<(usize, usize)>) -> bool {
        vec.iter().filter(|x| self.taken_space.contains(x)).count() > 0
    }

    fn move_rock_left(&mut self) -> () {
        let falling_rock = self.falling_rock.as_mut();
        let points = falling_rock.get_points();
        let wall_to_left = points.iter().map(|(x, _)| x == &0).count() > 0;
        if wall_to_left {
            return;
        }
        let vec = points.iter().map(|(x, y)| (*x - 1, *y)).collect_vec();
        if self.intersects(&vec) {
            self.falling_rock.set_points(vec);
        }
    }

    fn move_rock_right(&mut self) -> () {
        let falling_rock = self.falling_rock.as_mut();
        let points = falling_rock.get_points();
        let wall_to_right = points.iter().map(|(x, _)| x == &6).count() > 0;
        if wall_to_right {
            return;
        }
        let vec = points.iter().map(|(x, y)| (*x + 1, *y)).collect_vec();
        if self.intersects(&vec) {
            self.falling_rock.set_points(vec);
        }
    }

    fn move_rock_down(&mut self) -> Option<()> {
        let falling_rock = self.falling_rock.as_mut();
        let points = falling_rock.get_points();
        let floor_to_bottom = points.iter().map(|(_, y)| y == &0).count() > 0;
        if floor_to_bottom {
            return None;
        }
        let vec = points.iter().map(|(x, y)| (*x, *y - 1)).collect_vec();
        if self.intersects(&vec) {
            self.falling_rock.set_points(vec);
            Some(())
        } else {
            None
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
