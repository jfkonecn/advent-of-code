use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::{collections::*, iter::Map, str::Split};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
struct Tree {
    row: usize,
    col: usize,
    height: usize,
}

fn parse_file(file_contents: String) -> Vec<Tree> {
    file_contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .enumerate()
        .flat_map(|(row, str)| {
            str.chars().enumerate().map(move |(col, c)| Tree {
                height: String::from(c).parse::<usize>().unwrap(),
                col,
                row,
            })
        })
        .collect_vec()
}

fn insert_range<T: std::hash::Hash + Eq>(set: &mut HashSet<T>, iter: impl Iterator<Item = T>) {
    for x in iter {
        set.insert(x);
    }
}

fn visible_trees(trees: &Vec<Tree>) -> Vec<Tree> {
    let first_tree = trees.first().unwrap();
    let mut largest = first_tree.height;
    let mut visible = vec![first_tree.clone()];
    for tree in trees.iter().skip(1) {
        if largest < tree.height {
            visible.push(tree.clone());
            largest = tree.height;
        }
    }
    visible
}

fn visible_trees_sol_2(trees: &Vec<Tree>, starting: usize) -> Vec<Tree> {
    let first_tree = trees.first().unwrap();
    let to_take = trees.iter().take_while(|x| x.height < starting).count();
    trees.iter().take(to_take + 1).map(|x| *x).collect_vec()
}

fn visible_from_outside(trees: &Vec<Tree>) -> Vec<Tree> {
    let mut set: HashSet<Tree> = HashSet::new();
    let row_indexes = trees.iter().map(|x| x.row).unique();
    for row in row_indexes {
        add_visible(trees, &mut set, &|x: &Tree| x.row == row);
    }
    let col_indexes = trees.iter().map(|x| x.col).unique();
    for col in col_indexes {
        add_visible(trees, &mut set, &|x: &Tree| x.col == col);
    }
    Vec::from_iter(set)
}

fn add_visible(trees: &Vec<Tree>, set: &mut HashSet<Tree>, filter: &dyn Fn(&Tree) -> bool) {
    let vec = trees.into_iter().map(|x| *x).filter(filter).collect_vec();
    insert_range(set, visible_trees(&vec).into_iter());
    insert_range(
        set,
        visible_trees(&vec.into_iter().rev().collect_vec()).into_iter(),
    );
}

fn scenic_score(all_trees: &Vec<Tree>, tree: &Tree) -> usize {
    let mut total = 1;
    total *= check_visible_count(all_trees, false, tree.height, &|x: &Tree| {
        x.col > tree.col && x.row == tree.row
    });
    total *= check_visible_count(all_trees, true, tree.height, &|x: &Tree| {
        x.col < tree.col && x.row == tree.row
    });
    total *= check_visible_count(all_trees, true, tree.height, &|x: &Tree| {
        x.col == tree.col && x.row < tree.row
    });
    total *= check_visible_count(all_trees, false, tree.height, &|x: &Tree| {
        x.col == tree.col && x.row > tree.row
    });

    total
}

fn check_visible_count(
    all_trees: &Vec<Tree>,
    should_rev: bool,
    starting: usize,
    f: &dyn Fn(&Tree) -> bool,
) -> usize {
    let mut set: HashSet<Tree> = HashSet::new();

    let mut vec = all_trees.iter().map(|x| x.clone()).filter(f).collect_vec();
    if should_rev {
        vec.reverse();
    }
    if vec.len() > 0 {
        insert_range(&mut set, visible_trees_sol_2(&vec, starting).into_iter());
        set.len()
    } else {
        0
    }
}

pub fn solution_1(file_contents: String) -> usize {
    let trees = parse_file(file_contents);
    let visible = visible_from_outside(&trees);
    visible.len()
}

pub fn solution_2(file_contents: String) -> usize {
    let trees = parse_file(file_contents);
    let score = trees.iter().map(|x| scenic_score(&trees, x)).max().unwrap();
    score
}

challenge_test_suite!(
    solution_1,
    21,
    1698,
    solution_2,
    8,
    672280,
    "src",
    "year_2022",
    "day_8"
);
