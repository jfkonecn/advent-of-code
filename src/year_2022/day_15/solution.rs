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

    fn max_distance(&self) -> isize {
        let (s_x, s_y) = self.signal_point;
        let (b_x, b_y) = self.beacon_point;
        (s_x.abs_diff(b_x) + s_y.abs_diff(b_y)) as isize
    }

    fn get_x_range(&self, y: isize) -> Option<(isize, isize)> {
        let distance = self.max_distance();
        let (s_x, s_y) = self.signal_point;
        let distance_left = distance - s_y.abs_diff(y) as isize;
        println!("getting x range for ({}, {})", s_x, s_y);
        if distance_left < 0 {
            println!("out of range");
            None
        } else {
            let x_min = s_x - distance_left;
            let x_max = s_x + distance_left;
            println!("range {}..{}", x_min, x_max);
            Some((x_min, x_max))
        }
    }

    fn is_in_range(&self, (x, y): (isize, isize)) -> bool {
        let (s_x, s_y) = self.signal_point;
        let distance = self.max_distance();
        let distance_left = distance - s_y.abs_diff(y) as isize - s_x.abs_diff(x) as isize;
        distance_left >= 0
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
    // let set = pairs
    //     .iter()
    //     .map(|x| x.get_empty_points(y, is_solution_2))
    //     .fold(HashSet::new(), |mut acc, set| {
    //         for x in set {
    //             acc.insert(x);
    //         }
    //         acc
    //     });

    let beacon_and_signals: HashSet<_> = pairs
        .iter()
        .flat_map(|x| vec![x.signal_point, x.beacon_point])
        .collect();

    let mut set = HashSet::new();

    let iter = pairs.iter().filter_map(|x| x.get_x_range(y));

    if iter.clone().count() == 0 {
        return set;
    }

    let min_x = {
        let min_x = iter.clone().map(|(x, _)| x).min().unwrap();

        if is_solution_2 && min_x < 0 {
            0
        } else {
            min_x
        }
    };

    let max_x = {
        let max_x = iter.map(|(_, x)| x).max().unwrap();

        if is_solution_2 && max_x > 4000000 {
            4000000
        } else {
            max_x
        }
    };

    println!("{} .. {} + 1", min_x, max_x);

    for x in min_x..max_x + 1 {
        let point = (x, y);
        // println!("checking ({}, {})", x, y);
        if !is_solution_2 && beacon_and_signals.contains(&point) {
            println!("in b and s");
            continue;
        }
        for pair in pairs {
            if pair.is_in_range(point) {
                // println!("in range");
                set.insert(point);
            }
        }
    }
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
    // let y = 10;
    let set = beacon_not_in(&pairs, y, false);
    set.len()
}

pub fn solution_2(file_contents: String) -> isize {
    let pairs = parse_map(file_contents);
    let min_y = pairs
        .iter()
        .map(|x| x.signal_point.1 - x.max_distance())
        .min()
        .unwrap()
        .max(0);

    let max_y = pairs
        .iter()
        .map(|x| x.signal_point.1 + x.max_distance())
        .max()
        .unwrap()
        .min(4000000);
    println!("{} .. {} + 1", min_y, max_y);
    for y in min_y..max_y + 1 {
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
