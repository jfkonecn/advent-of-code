use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solution_1(file_contents: String) -> usize {
    file_contents
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|str| {
            let idx = str.len() / 2;
            let a = &str[..idx];
            let b = &str[idx..];
            let mut set = HashSet::new();
            for c in a.chars() {
                set.insert(c);
            }
            for c in b.chars() {
                if set.contains(&c) {
                    return c;
                }
            }
            panic!("no matches between \"{}\" and \"{}\"", a, b)
        })
        .map(char_to_value)
        .sum()
}

fn char_to_value(c: char) -> usize {
    let small_a = 'a' as usize;
    let big_a = 'A' as usize;
    let char_num = c as usize;
    let num = if char_num >= small_a {
        char_num - small_a + 1
    } else {
        char_num - big_a + 27
    };
    num
}

pub fn solution_2(file_contents: String) -> usize {
    file_contents
        .split("\n")
        .filter(|x| !x.is_empty())
        .enumerate()
        .group_by(|(idx, _)| idx / 3)
        .into_iter()
        .map(|(_, group)| {
            group
                .into_iter()
                .map(|(_, x)| x)
                .flat_map(|x| x.chars().map(char_to_value).unique())
                .counts()
                .into_iter()
                .filter(|(_, x)| *x == 3usize)
                .map(|(x, _)| x)
                .next()
                .unwrap()
        })
        .sum()
}

challenge_test_suite!(
    solution_1,
    157,
    8394,
    solution_2,
    70,
    2413,
    "src",
    "year_2022",
    "day_3"
);
