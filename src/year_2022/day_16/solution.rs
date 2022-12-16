use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::cmp::*;
use std::collections::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RawValve {
    id: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

impl From<&str> for RawValve {
    fn from(str: &str) -> Self {
        let id = str[6..8].to_owned();
        let flow_rate = str
            .split('=')
            .last()
            .unwrap()
            .split(';')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let tunnels = str
            .split("valves")
            .last()
            .unwrap()
            .split("valve")
            .last()
            .unwrap()
            .split(',')
            .map(|x| x.trim().to_owned())
            .collect_vec();
        RawValve {
            id,
            flow_rate,
            tunnels,
        }
    }
}

fn parse_valves(file_contents: String) -> Vec<RawValve> {
    file_contents
        .split("\n")
        .filter(|x| !x.is_empty())
        .map_into()
        .collect_vec()
}
pub fn solution_1(file_contents: String) -> usize {
    let valves = parse_valves(file_contents);
    println!("{:?}", valves);
    0
}

pub fn solution_2(file_contents: String) -> usize {
    0
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
    "day_16"
);
