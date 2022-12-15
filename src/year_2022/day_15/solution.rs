use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::cmp::*;
use std::collections::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SBPair {
    signal_point: (isize, isize),
    beacon_point: (isize, isize),
}

impl SBPair {
    fn get_empty_points(&self, y: isize) -> HashSet<(isize, isize)> {
        let mut set = HashSet::new();
        let (s_x, s_y) = self.signal_point;
        let (b_x, b_y) = self.beacon_point;
        let distance = (s_x.abs_diff(b_x) + s_y.abs_diff(b_y)) as isize;
        let x_distance = distance - s_y.abs_diff(y) as isize;
        for x in s_x - x_distance..s_x + x_distance + 1 {
            if !(x == s_x && y == s_y) && !(x == b_x && y == b_y) {
                set.insert((x, y));
            }
        }
        set
    }
}

fn parse_map(str: String) -> Vec<SBPair> {
    str.split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut iter = x.split("=");
            iter.next();
            let s_x: isize = iter
                .next()
                .unwrap()
                .split(',')
                .next()
                .unwrap()
                .trim()
                .parse()
                .unwrap();
            let s_y: isize = iter
                .next()
                .unwrap()
                .split(':')
                .next()
                .unwrap()
                .trim()
                .parse()
                .unwrap();
            let b_x: isize = iter
                .next()
                .unwrap()
                .split(',')
                .next()
                .unwrap()
                .trim()
                .parse()
                .unwrap();
            let b_y: isize = iter
                .next()
                .unwrap()
                .split(':')
                .next()
                .unwrap()
                .trim()
                .parse()
                .unwrap();
            SBPair {
                signal_point: (s_x, s_y),
                beacon_point: (b_x, b_y),
            }
        })
        .collect_vec()
}

pub fn solution_1(file_contents: String) -> usize {
    let pairs = parse_map(file_contents);
    let y = 2000000;
    let set = pairs
        .iter()
        .map(|x| x.get_empty_points(y))
        .fold(HashSet::new(), |mut acc, set| {
            for x in set {
                acc.insert(x);
            }
            acc
        });
    set.len()
}

pub fn solution_2(file_contents: String) -> usize {
    let mut map = parse_map(file_contents);
    println!("{:?}", map);
    1
}

challenge_test_suite!(
    solution_1,
    0,
    5878678,
    solution_2,
    1,
    1,
    "src",
    "year_2022",
    "day_15"
);
