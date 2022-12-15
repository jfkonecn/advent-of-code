use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::cmp::*;
use std::collections::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MapType {
    Empty,
    Sensor,
    Beacon,
}

struct SBPair {
    signal_point: (isize, isize),
    beacon_point: (isize, isize),
}

impl SBPair {
    fn get_empty_points(&self, row: isize) -> HashSet<(isize, isize)> {
        let set = HashSet::new();
        set
    }
}

fn parse_map(str: String) -> HashMap<(isize, isize), MapType> {
    let mut hash_map = HashMap::new();
    str.split("\n").filter(|x| !x.is_empty()).for_each(|x| {
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
        println!("--------------");
        // println!("Sensor ({}, {}) - Beacon ({}, {})", s_x, s_y, b_x, b_y);
        let distance = (s_x.abs_diff(b_x) + s_y.abs_diff(b_y)) as isize;
        for y in s_y - distance..s_y + distance + 1 {
            let x_distance = distance - s_y.abs_diff(y) as isize;
            for x in s_x - x_distance..s_x + x_distance + 1 {
                let map_type = if x == s_x && y == s_y {
                    MapType::Sensor
                } else if x == b_x && y == b_y {
                    MapType::Beacon
                } else {
                    MapType::Empty
                };
                println!("({}, {}) - {:?}", x, y, map_type);
                hash_map.insert((x, y), map_type);
            }
        }
        println!("--------------");
    });
    hash_map
}

pub fn solution_1(file_contents: String) -> usize {
    let mut map = parse_map(file_contents);
    println!("{:?}", map);
    1
}

pub fn solution_2(file_contents: String) -> usize {
    let mut map = parse_map(file_contents);
    println!("{:?}", map);
    1
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
    "day_15"
);
