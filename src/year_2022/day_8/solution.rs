use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::{collections::*, iter::Map, str::Split};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
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

fn visible_trees(vec: &Vec<Tree>) -> impl Iterator<Item = Tree> + '_ {
    let to_take = vec
        .iter()
        .zip(vec.iter().skip(1))
        .take_while(|(l, r)| l.height < r.height)
        .count();
    vec.into_iter().take(to_take + 1).map(|x| x.clone())
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
    insert_range(set, visible_trees(&vec));
    insert_range(set, visible_trees(&vec.into_iter().rev().collect_vec()));
}

pub fn solution_1(file_contents: String) -> usize {
    let trees = parse_file(file_contents);
    let visible = visible_from_outside(&trees);
    // println!("{:?}", trees);
    let mut deb = visible
        .clone()
        .into_iter()
        .map(|x| (x.row, x.col, x.height))
        .collect_vec();
    deb.sort();
    println!("{:?}", deb);
    visible.len()
}

pub fn solution_2(file_contents: String) -> usize {
    1
}

challenge_test_suite!(
    solution_1,
    21,
    1,
    solution_2,
    1,
    1,
    "src",
    "year_2022",
    "day_8"
);
