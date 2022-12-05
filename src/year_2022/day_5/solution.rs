use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
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

fn parse_crates_and_commands<'a>(file_contents: String) -> (Vec<&'a mut Vec<Crate>>, Vec<Command>) {
    let str_vec = file_contents.split("\n\n").collect_vec();
    let mut crates_str_vec = str_vec[0]
        .split('\n')
        // .filter(|x| !x.is_empty())
        .collect_vec();
    let mut crate_vec: Vec<Vec<Crate>> = {
        let temp = crates_str_vec.remove(crates_str_vec.len() - 1);
        parse_line_of_crates(temp)
            .iter()
            .map(|_| vec![])
            .collect_vec()
    };
    println!("{:?}", crates_str_vec);
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

    let crate_vec: Vec<&'a mut Vec<Crate>> = {
        let mut temp: Vec<&'a mut Vec<Crate>> = vec![];
        for v in crate_vec {
            let mut new_v = vec![];
            for c in v.iter().rev().map(|x| x.clone()) {
                new_v.push(c);
            }
            temp.push(&mut new_v);
        }
        temp
    };

    let command_vec = str_vec[1]
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| Command::from(x))
        .collect_vec();
    (crate_vec, command_vec)
}

pub fn solution_1(file_contents: String) -> usize {
    let (crates, commands) = parse_crates_and_commands(file_contents);
    println!("{:?}", crates);
    println!("{:?}", commands);

    commands.iter().for_each(
        |Command {
             amount,
             from_idx,
             to_idx,
         }| {
            let &mut from = &mut crates.get(*from_idx).unwrap();
            let &mut to = &mut crates.get(*to_idx).unwrap();
            for _ in 0..*amount {
                let temp = from.pop().unwrap();
                to.push(temp);
            }
        },
    );

    println!("{:?}", crates);
    2
}

pub fn solution_2(file_contents: String) -> usize {
    2
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
    "day_5"
);
