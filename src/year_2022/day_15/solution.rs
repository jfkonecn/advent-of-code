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
        if distance_left < 0 {
            None
        } else {
            let x_min = s_x - distance_left;
            let x_max = s_x + distance_left;
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

fn beacon_not_in(pairs: &Vec<SBPair>, y: isize, is_solution_2: bool) -> (isize, Option<isize>) {
    let iter = pairs.iter().filter_map(|x| x.get_x_range(y)).sorted();

    if iter.clone().count() == 0 {
        return (0, None);
    }

    let mut count = 0;

    if is_solution_2 {
        let seed = iter.clone().map(|(x, _)| x).min().unwrap().max(0);
        let result = iter
            .sorted()
            .map(|(x, y)| {
                if x < 0 {
                    (0, y)
                } else if x > 4000000 {
                    (4000000, y)
                } else {
                    (x, y)
                }
            })
            .fold(((seed, seed), None), |acc, a| {
                let ((x_s_1, x_b_1), opt) = acc;
                let (x_s_2, x_b_2) = a;
                if x_b_1 - x_s_2 == -2 {
                    ((x_b_1, x_s_2), Some(x_b_1 + 1))
                } else {
                    ((x_s_1, x_b_2.max(x_b_1)), opt)
                }
            })
            .1;
        (count, result)
    } else {
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
        let beacon_and_signals: HashSet<_> = pairs
            .iter()
            .flat_map(|x| vec![x.signal_point, x.beacon_point])
            .collect();
        for x in min_x..max_x + 1 {
            let point = (x, y);
            if !is_solution_2 && beacon_and_signals.contains(&point) {
                continue;
            }
            for pair in pairs {
                if pair.is_in_range(point) {
                    // actual_sum += x;
                    count += 1;
                    break;
                }
            }
        }

        (count, None)
    }
}

fn check_if_gap(pairs: &Vec<SBPair>, y: isize, is_solution_2: bool) -> Option<isize> {
    let (_, x) = beacon_not_in(&pairs, y, is_solution_2);
    x
}

pub fn solution_1(file_contents: String) -> usize {
    let pairs = parse_map(file_contents);
    let y = 2000000;
    let (len, _) = beacon_not_in(&pairs, y, false);
    len as usize
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
    for y in min_y..max_y + 1 {
        println!("testing y : {}", y);
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
    11796491041245,
    "src",
    "year_2022",
    "day_15"
);
