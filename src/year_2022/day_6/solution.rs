use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::collections::VecDeque;
use std::{borrow::Borrow, collections::HashSet};

#[derive(Debug, Clone)]
struct Crate {
    label: char,
}

fn parse_crate<'a>(c: char) -> Option<Crate> {
    if c.is_alphabetic() {
        Some(Crate { label: c })
    } else {
        None
    }
}

fn parse_line_of_crates(str: &str) -> Vec<Option<Crate>> {
    str.chars()
        .skip(1)
        .step_by(4)
        .map(|x| parse_crate(x))
        .collect_vec()
}

#[derive(Debug, Clone)]
struct Command {
    amount: usize,
    from_idx: usize,
    to_idx: usize,
}

impl From<&str> for Command {
    fn from(str: &str) -> Self {
        let vec = str.split(' ').collect_vec();
        Command {
            amount: (*vec.get(1).unwrap()).parse().unwrap(),
            from_idx: (*vec.get(3).unwrap()).parse::<usize>().unwrap() - 1usize,
            to_idx: (*vec.get(5).unwrap()).parse::<usize>().unwrap() - 1usize,
        }
    }
}

fn parse_crates_and_commands(file_contents: String) -> (Vec<Vec<Crate>>, Vec<Command>) {
    let str_vec = file_contents.split("\n\n").collect_vec();
    let mut crates_str_vec = str_vec[0].split('\n').collect_vec();
    let mut crate_vec: Vec<Vec<Crate>> = {
        let temp = crates_str_vec.remove(crates_str_vec.len() - 1);
        parse_line_of_crates(temp)
            .iter()
            .map(|_| vec![])
            .collect_vec()
    };
    crates_str_vec
        .iter()
        .map(|x| parse_line_of_crates(x))
        .for_each(|v| {
            v.iter().enumerate().for_each(|(idx, x)| {
                if let Some(c) = x {
                    crate_vec[idx].push((*c).clone());
                }
            })
        });

    crate_vec.iter_mut().for_each(|x| x.reverse());

    let command_vec = str_vec[1]
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| Command::from(x))
        .collect_vec();
    (crate_vec, command_vec)
}

fn end_of_message_idx(file_contents: String, msg_size: usize) -> usize {
    let start_idx = msg_size - 1;
    let mut queue: VecDeque<char> = file_contents[..start_idx].chars().collect();
    for (idx, c) in file_contents[start_idx..].chars().enumerate() {
        queue.push_back(c);
        let count = queue.iter().unique().count();
        if count == msg_size {
            return idx + msg_size;
        }
        queue.pop_front();
    }
    unreachable!("Something went wrong")
}

pub fn solution_1(file_contents: String) -> usize {
    end_of_message_idx(file_contents, 4)
}

pub fn solution_2(file_contents: String) -> usize {
    end_of_message_idx(file_contents, 14)
}

challenge_test_suite!(
    solution_1,
    10,
    1802,
    solution_2,
    29,
    3551,
    "src",
    "year_2022",
    "day_6"
);
