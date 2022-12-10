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

fn parse_commands(file_contents: String) -> Vec<Command> {
    file_contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .map_into()
        .collect_vec()
}

fn simulate(commands: Vec<Command>) -> Vec<isize> {
    let mut system_value = 1;
    let mut history = vec![system_value];
    for command in commands {
        match command {
            Command::Noop => {
                history.push(system_value);
            }
            Command::AddX(value) => {
                history.push(system_value);

                system_value += value;
                history.push(system_value);
            }
        };
    }
    history
}

fn get_signal_strength(history: &Vec<isize>, cycles: Vec<usize>) -> isize {
    history
        .iter()
        .enumerate()
        .filter_map(|(idx, system_value)| -> Option<isize> {
            let during_cycle = idx + 1;
            if cycles.contains(&during_cycle) {
                let cycle: isize = during_cycle.try_into().unwrap();
                let result = cycle * system_value;
                println!("{}, {} * {} = {}", idx, cycle, system_value, result);
                Some(result)
            } else {
                None
            }
        })
        .sum()
}

fn draw(history: &Vec<isize>) -> String {
    let mut picture = String::from("");
    for (idx, value) in history.iter().enumerate() {
        let idx = isize::try_from(idx).unwrap();
        let px = idx % 40;
        if px == value - 1 || px == *value || px == value + 1 {
            picture.push('#');
        } else {
            picture.push('.');
        }
        if idx % 40 == 39 {
            picture.push('\n');
        }
    }
    picture
}

pub fn solution_1(file_contents: String) -> isize {
    let commands = parse_commands(file_contents);
    let history = simulate(commands);
    get_signal_strength(&history, vec![20, 60, 100, 140, 180, 220])
}

pub fn solution_2(file_contents: String) -> String {
    let commands = parse_commands(file_contents);
    let history = simulate(commands);
    let picture = draw(&history);
    println!("{}", picture);
    picture
}

challenge_test_suite!(
    solution_1,
    13140,
    13680,
    solution_2,
    "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n.",
    "###..####..##..###..#..#.###..####.###..\n#..#....#.#..#.#..#.#.#..#..#.#....#..#.\n#..#...#..#....#..#.##...#..#.###..###..\n###...#...#.##.###..#.#..###..#....#..#.\n#....#....#..#.#....#.#..#....#....#..#.\n#....####..###.#....#..#.#....####.###..\n.",
    "src",
    "year_2022",
    "day_10"
);
