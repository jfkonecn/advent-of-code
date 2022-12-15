use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::cmp::*;
use std::collections::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CaveItemType {
    Rock,
    Sand,
}

fn parse_rocks(str: String) -> HashMap<(usize, usize), CaveItemType> {
    let mut hash_map = HashMap::new();
    str.split("\n").filter(|x| !x.is_empty()).for_each(|x| {
        let mut iter = x.split(" -> ");
        let mut pre_point = iter.next().unwrap().split(",");
        let mut pre_x: usize = pre_point.next().unwrap().trim().parse().unwrap();
        let mut pre_y: usize = pre_point.next().unwrap().trim().parse().unwrap();
        while let Some(point_str) = iter.next() {
            let mut point = point_str.split(",");
            println!("\"{}\"", point_str);
            let x: usize = point.next().unwrap().trim().parse().unwrap();
            let y: usize = point.next().unwrap().trim().parse().unwrap();

            (if x == pre_x {
                let min_y = y.min(pre_y);
                let max_y = y.max(pre_y);
                (min_y..max_y + 1).map(|y| (x, y)).collect_vec()
            } else {
                let min_x = x.min(pre_x);
                let max_x = x.max(pre_x);
                (min_x..max_x + 1).map(|x| (x, y)).collect_vec()
            })
            .into_iter()
            .for_each(|x| {
                hash_map.insert(x, CaveItemType::Rock);
            });
            pre_x = x;
            pre_y = y;
        }
    });
    hash_map
}

pub fn solution_1(file_contents: String) -> usize {
    let rocks = parse_rocks(file_contents);
    println!(
        "{:?}",
        rocks.into_iter().map(|(x, _)| x).sorted().collect_vec()
    );
    1
}

pub fn solution_2(file_contents: String) -> usize {
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
    "day_14"
);
