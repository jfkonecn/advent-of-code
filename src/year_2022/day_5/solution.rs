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

pub fn solution_1(file_contents: String) -> String {
    let (mut crates, commands) = parse_crates_and_commands(file_contents);

    commands.iter().for_each(
        |Command {
             amount,
             from_idx,
             to_idx,
         }| {
            for _ in 0..*amount {
                let from = crates.get_mut(*from_idx).unwrap();
                let temp = from.pop().unwrap();
                let to = crates.get_mut(*to_idx).unwrap();
                to.push(temp);
            }
        },
    );

    crates.iter().map(|x| x.last().unwrap().label).join("")
}

pub fn solution_2(file_contents: String) -> String {
    let (mut crates, commands) = parse_crates_and_commands(file_contents);

    commands.iter().for_each(
        |Command {
             amount,
             from_idx,
             to_idx,
         }| {
            let mut storage = VecDeque::new();
            let from = crates.get_mut(*from_idx).unwrap();
            for _ in 0..*amount {
                let temp = from.pop().unwrap();
                storage.push_front(temp);
            }
            let to = crates.get_mut(*to_idx).unwrap();
            for c in storage {
                to.push(c);
            }
        },
    );

    crates.iter().map(|x| x.last().unwrap().label).join("")
}

challenge_test_suite!(
    solution_1,
    "CMZ",
    "RFFFWBPNS",
    solution_2,
    "MCD",
    "CQQBBJFCS",
    "src",
    "year_2022",
    "day_5"
);
