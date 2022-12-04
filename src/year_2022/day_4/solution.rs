use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
struct Assignment {
    Low: usize,
    High: usize,
}

impl From<&str> for Assignment {
    fn from(str: &str) -> Self {
        let vec = str.split('-').collect::<Vec<&str>>();
        Assignment {
            Low: (*vec.get(0).unwrap()).parse().unwrap(),
            High: (*vec.get(1).unwrap()).parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct AssignmentPair {
    left: Assignment,
    right: Assignment,
}

impl From<&str> for AssignmentPair {
    fn from(str: &str) -> Self {
        let vec = str.split(',').collect::<Vec<&str>>();
        AssignmentPair {
            left: (*vec.get(0).unwrap()).into(),
            right: (*vec.get(1).unwrap()).into(),
        }
    }
}

impl AssignmentPair {
    pub fn is_subset(&self) -> bool {
        let ref left = self.left;
        let ref right = self.right;
        (left.Low <= right.Low && right.High <= left.High)
            || (right.Low <= left.Low && left.High <= right.High)
    }

    pub fn has_overlap(&self) -> bool {
        let ref left = self.left;
        let ref right = self.right;
        (left.Low <= right.Low && right.Low <= left.High)
            || (left.Low <= right.High && right.High <= left.High)
            || (right.Low <= left.Low && left.Low <= right.High)
            || (right.Low <= left.High && left.High <= right.High)
    }
}

pub fn solution_1(file_contents: String) -> usize {
    file_contents
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(AssignmentPair::from)
        .filter(|x| x.is_subset())
        .count()
}

pub fn solution_2(file_contents: String) -> usize {
    file_contents
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(AssignmentPair::from)
        .filter(|x| x.has_overlap())
        .count()
}

challenge_test_suite!(
    solution_1,
    2,
    536,
    solution_2,
    4,
    845,
    "src",
    "year_2022",
    "day_4"
);
