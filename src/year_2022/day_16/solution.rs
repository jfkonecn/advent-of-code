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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tunnel {
    to: String,
    flow_rate: usize,
    distance: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    id: String,
    flow_rate: usize,
    tunnels: Vec<Tunnel>,
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

fn parse_raw_valves(file_contents: String) -> Vec<RawValve> {
    file_contents
        .split("\n")
        .filter(|x| !x.is_empty())
        .map_into()
        .collect_vec()
}

fn get_tunnels(raw_valve: &RawValve, graph: &Vec<RawValve>) -> Vec<Tunnel> {
    graph
        .iter()
        .filter(|x| x.id != raw_valve.id && x.flow_rate > 0)
        .map(|goal| {
            let (_, distance) = pathfinding::directed::dijkstra::dijkstra(
                raw_valve,
                |cur_value| {
                    graph
                        .iter()
                        .filter(|x| x.id == cur_value.id)
                        .next()
                        .unwrap()
                        .tunnels
                        .iter()
                        .map(|id| graph.iter().filter(|x| &x.id == id).next().unwrap())
                        .map(|x| (x.clone(), 1))
                        .collect_vec()
                        .clone()
                },
                |x| x.id == goal.id,
            )
            .unwrap();
            Tunnel {
                distance,
                flow_rate: goal.flow_rate,
                to: goal.id.clone(),
            }
        })
        .collect_vec()
}

fn to_valves(raw_valves: &Vec<RawValve>) -> Vec<Valve> {
    raw_valves
        .iter()
        .map(|x| Valve {
            flow_rate: x.flow_rate,
            id: x.id.clone(),
            tunnels: get_tunnels(x, raw_valves),
        })
        .collect_vec()
}

fn max_pressure_rec(
    cur_valve: &Valve,
    graph: &Vec<Valve>,
    time_left: usize,
    visited: &Vec<String>,
) -> Vec<(usize, Vec<String>)> {
    let visited = {
        let mut visited = visited.clone();
        visited.push(cur_valve.id.clone());
        visited
    };
    if time_left < 2 {
        return vec![(0, visited)];
    }

    let results = cur_valve
        .tunnels
        .iter()
        .filter(|x| time_left > x.distance + 1 && !visited.contains(&x.to))
        .flat_map(|tunnel| {
            let next_valve = graph.iter().find(|x| x.id == tunnel.to).unwrap();
            let time_left_new = time_left - tunnel.distance - 1;

            let t = max_pressure_rec(next_valve, graph, time_left_new, &visited)
                .into_iter()
                .map(|(pressure, vec)| {
                    let pressure_delta = time_left_new * tunnel.flow_rate;
                    let pressure = pressure + pressure_delta;
                    (pressure, vec.clone())
                })
                .collect_vec();
            t
        })
        .collect_vec();
    if results.len() == 0 {
        return vec![(0, visited)];
    } else {
        results
    }
}

fn max_pressure(cur_valve: &Valve, graph: &Vec<Valve>) -> usize {
    let results = max_pressure_rec(cur_valve, graph, 30, &vec![]);
    // println!("{:#?}", results);
    let (pressure, vec) = results.iter().max_by_key(|x| x.0).unwrap();
    println!("{:#?}", vec);
    *pressure
}

fn max_pressure_rec_part_2(
    cur_valves: &Vec<(usize, Valve)>,
    graph: &Vec<Valve>,
    visited: &Vec<String>,
) -> Vec<(usize, Vec<String>)> {
    let visited = {
        let mut visited = visited.clone();
        visited.append(&mut cur_valves.iter().map(|(_, x)| x.id.clone()).collect_vec());
        visited
    };
    if cur_valves.iter().any(|(time_left, _)| time_left < &2usize) {
        return vec![(0, visited)];
    }

    let results = graph
        .iter()
        .filter(|x| !visited.contains(&x.id))
        .combinations_with_replacement(2)
        .flat_map(|x| vec![x.clone(), x.clone().into_iter().rev().collect_vec()])
        .filter(|x| x.iter().all_unique())
        .flat_map(|valves| {
            let mut iter = valves
                .into_iter()
                .enumerate()
                .map(|(idx, valve)| {
                    graph
                        .iter()
                        .find(|x| x.id == valve.id)
                        .unwrap()
                        .tunnels
                        .iter()
                        .map(move |x| {
                            let (time_left, _) = cur_valves.get(idx).unwrap();
                            (*time_left, x)
                        })
                })
                .filter(|x| {
                    x.clone().all(|(time_left, x)| {
                        time_left > x.distance + 1 && !visited.contains(&x.to)
                    })
                });

            if iter.clone().count() != 2 {
                return vec![];
            }

            let first = iter.next().unwrap();
            iter.next()
                .unwrap()
                .into_iter()
                .zip(first)
                .map(|(x, y)| vec![x, y])
                .collect_vec()
        })
        .flat_map(|tunnels| {
            let next_valves = tunnels
                .into_iter()
                .map(|(time_left, tunnel)| {
                    let time_left_new = time_left - tunnel.distance - 1;
                    let next_valve = graph.iter().find(|x| x.id == tunnel.to).unwrap().clone();
                    (time_left_new, next_valve)
                })
                .collect_vec();

            let t = max_pressure_rec_part_2(&next_valves, graph, &visited)
                .into_iter()
                .map(|(pressure, vec)| {
                    // let pressure_delta = time_left_new * tunnels.flow_rate;
                    let pressure_delta: usize = cur_valves
                        .iter()
                        .map(|(time_left, valve)| time_left * valve.flow_rate)
                        .sum();

                    let pressure = pressure + pressure_delta;
                    (pressure, vec.clone())
                })
                .collect_vec();
            t
        })
        .collect_vec();
    if results.len() == 0 {
        return vec![(0, visited)];
    } else {
        results
    }
}

fn max_pressure_part_2(cur_valve: &Valve, graph: &Vec<Valve>) -> usize {
    let results = max_pressure_rec_part_2(
        &vec![(26, cur_valve.clone()), (26, cur_valve.clone())],
        graph,
        &vec![],
    );
    // println!("{:#?}", results);
    let (pressure, vec) = results.iter().max_by_key(|x| x.0).unwrap();
    println!("{:#?}", vec);
    *pressure
}

pub fn solution_1(file_contents: String) -> usize {
    let raw_valves = parse_raw_valves(file_contents);
    let valves = to_valves(&raw_valves);
    valves
        .iter()
        .filter(|x| x.id == "AA")
        .map(|x| max_pressure(x, &valves))
        .max()
        .unwrap()
}

pub fn solution_2(file_contents: String) -> usize {
    let raw_valves = parse_raw_valves(file_contents);
    let valves = to_valves(&raw_valves);
    valves
        .iter()
        .filter(|x| x.id == "AA")
        .map(|x| max_pressure_part_2(x, &valves))
        .max()
        .unwrap()
}

challenge_test_suite!(
    solution_1,
    1651,
    1474,
    solution_2,
    1707,
    1,
    "src",
    "year_2022",
    "day_16"
);
