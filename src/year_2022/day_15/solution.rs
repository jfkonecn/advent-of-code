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
    fn get_empty_points(&self, y: isize, is_solution_2: bool) -> HashSet<(isize, isize)> {
        let mut set = HashSet::new();
        let (s_x, s_y) = self.signal_point;
        let (b_x, b_y) = self.beacon_point;
        let distance = (s_x.abs_diff(b_x) + s_y.abs_diff(b_y)) as isize;
        let x_distance = distance - s_y.abs_diff(y) as isize;
        let min = if is_solution_2 { 0 } else { isize::MIN };
        let max = if is_solution_2 { 4000000 } else { isize::MAX };
        for x in s_x - x_distance..s_x + x_distance + 1 {
            if x < min {
                continue;
            } else if x > max {
                break;
            } else if (!(x == s_x && y == s_y) && !(x == b_x && y == b_y)) || is_solution_2 {
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

fn beacon_not_in(pairs: &Vec<SBPair>, y: isize, is_solution_2: bool) -> HashSet<(isize, isize)> {
    let set = pairs
        .iter()
        .map(|x| x.get_empty_points(y, is_solution_2))
        .fold(HashSet::new(), |mut acc, set| {
            for x in set {
                acc.insert(x);
            }
            acc
        });
    set
}

fn check_if_gap(pairs: &Vec<SBPair>, y: isize, is_solution_2: bool) -> Option<isize> {
    let mut iter = beacon_not_in(&pairs, y, is_solution_2).into_iter().sorted();
    let (mut pre_x, _) = iter.next().unwrap();
    while let Some((x, _)) = iter.next() {
        println!("({}, {}) -> ({}, {})", pre_x, y, x, y);
        if pre_x + 1 < x {
            return Some(pre_x + 1);
        }
        pre_x = x;
    }
    None
}

pub fn solution_1(file_contents: String) -> usize {
    let pairs = parse_map(file_contents);
    let y = 2000000;
    let set = beacon_not_in(&pairs, y, false);
    set.len()
}

pub fn solution_2(file_contents: String) -> isize {
    let pairs = parse_map(file_contents);
    let min = 0;
    let max = 4000000;
    for y in min..max + 1 {
        let opt_x = check_if_gap(&pairs, y, true);
        if let Some(x) = opt_x {
            println!("answer ({}, {})", x, y);
            return (4000000 * x) + y;
        }
    }

    -1
}

challenge_test_suite!(
    solution_1,
    0,
    5878678,
    solution_2,
    56000011,
    1,
    "src",
    "year_2022",
    "day_15"
);
