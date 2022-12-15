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

fn total_rocks_stacked(
    mut rocks: HashMap<(usize, usize), CaveItemType>,
    is_solution_2: bool,
) -> usize {
    let mut total_rocks_stacked = 0;
    let deepest_point = *rocks.iter().map(|((_, x), _)| x).max().unwrap();
    let mut should_continue = true;
    while should_continue {
        let (mut cur_x, mut cur_y) = (500, 0);
        if rocks.contains_key(&(cur_x, cur_y)) {
            break;
        }
        should_continue = false;
        while cur_y < deepest_point + 3 {
            if !rocks.contains_key(&(cur_x, cur_y + 1)) {
                cur_y += 1;
            } else if !rocks.contains_key(&(cur_x - 1, cur_y + 1)) {
                cur_x -= 1;
                cur_y += 1;
            } else if !rocks.contains_key(&(cur_x + 1, cur_y + 1)) {
                cur_x += 1;
                cur_y += 1;
            } else {
                rocks.insert((cur_x, cur_y), CaveItemType::Sand);
                total_rocks_stacked += 1;
                should_continue = true;
                break;
            }
            if is_solution_2 && cur_y == deepest_point + 1 {
                rocks.insert((cur_x, cur_y), CaveItemType::Sand);
                total_rocks_stacked += 1;
                should_continue = true;
                break;
            }
        }
    }
    total_rocks_stacked
}

pub fn solution_1(file_contents: String) -> usize {
    let mut rocks = parse_rocks(file_contents);
    total_rocks_stacked(rocks, false)
}

pub fn solution_2(file_contents: String) -> usize {
    let mut rocks = parse_rocks(file_contents);

    total_rocks_stacked(rocks, true)
}

challenge_test_suite!(
    solution_1,
    24,
    715,
    solution_2,
    93,
    25248,
    "src",
    "year_2022",
    "day_14"
);
