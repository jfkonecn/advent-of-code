use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::{borrow::Borrow, collections::*};

#[derive(Debug, Clone)]
enum Command {
    Noop,
    AddX(isize),
}

impl<'a> From<&str> for Command {
    fn from(str: &str) -> Self {
        let str = str.to_owned();
        let mut raw_strs = str.split_ascii_whitespace().filter(|x| !x.is_empty());
        let cmd_str = raw_strs.next().unwrap();

        match cmd_str {
            "addx" => {
                let amount = raw_strs.next().unwrap().parse().unwrap();
                Command::AddX(amount)
            }
            "noop" => Command::Noop,
            _ => unreachable!("Unknown command {}", cmd_str),
        }
    }
}

#[derive(Debug, Clone)]
struct SystemState {
    end_of_cycle: usize,
    value: isize,
}

fn parse_commands(file_contents: String) -> Vec<Command> {
    file_contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .map_into()
        .collect_vec()
}

fn simulate(commands: Vec<Command>) -> Vec<SystemState> {
    let mut system_value = 1;
    let mut end_of_cycle = 0;
    let mut history = vec![SystemState {
        value: system_value,
        end_of_cycle,
    }];
    for command in commands {
        match command {
            Command::Noop => {
                end_of_cycle += 1;
                history.push(SystemState {
                    end_of_cycle,
                    value: system_value,
                });
            }
            Command::AddX(value) => {
                end_of_cycle += 1;
                history.push(SystemState {
                    end_of_cycle,
                    value: system_value,
                });

                end_of_cycle += 1;
                system_value += value;
                history.push(SystemState {
                    end_of_cycle,
                    value: system_value,
                });
            }
        };
    }
    history
}

fn get_signal_strength(history: &Vec<SystemState>, cycles: Vec<usize>) -> isize {
    history
        .iter()
        .filter_map(|x| -> Option<isize> {
            if x.end_of_cycle == 0 {
                return None;
            }
            let during_cycle = x.end_of_cycle + 1;
            if cycles.contains(&during_cycle) {
                let cycle: isize = during_cycle.try_into().unwrap();
                let result = cycle * x.value;
                println!("{}, {} * {} = {}", x.end_of_cycle, cycle, x.value, result);
                Some(result)
            } else {
                None
            }
        })
        .sum()
}

pub fn solution_1(file_contents: String) -> isize {
    let commands = parse_commands(file_contents);
    let history = simulate(commands);
    get_signal_strength(&history, vec![20, 60, 100, 140, 180, 220])
}

pub fn solution_2(file_contents: String) -> isize {
    let commands = parse_commands(file_contents);
    let history = simulate(commands);
    1
}

challenge_test_suite!(
    solution_1,
    13140,
    13680,
    solution_2,
    1,
    1,
    "src",
    "year_2022",
    "day_10"
);
