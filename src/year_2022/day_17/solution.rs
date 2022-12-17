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
    limit: usize,
) -> Vec<(usize, Vec<String>)> {
    let visited = {
        let mut visited = visited.clone();
        visited.push(cur_valve.id.clone());
        visited
    };
    if time_left < 2 || limit < visited.len() {
        return vec![(0, visited)];
    }

    let results = cur_valve
        .tunnels
        .iter()
        .filter(|x| time_left > x.distance + 1 && !visited.contains(&x.to))
        .flat_map(|tunnel| {
            let next_valve = graph.iter().find(|x| x.id == tunnel.to).unwrap();
            let time_left_new = time_left - tunnel.distance - 1;

            let t = max_pressure_rec(next_valve, graph, time_left_new, &visited, limit)
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
    let results = max_pressure_rec(cur_valve, graph, 30, &vec![], usize::MAX);
    // println!("{:#?}", results);
    let (pressure, vec) = results.iter().max_by_key(|x| x.0).unwrap();
    println!("{:#?}", vec);
    *pressure
}

fn max_pressure_part_2(cur_valve: &Valve, graph: &Vec<Valve>) -> usize {
    let total_flows = graph
        .iter()
        .filter(|x| x.flow_rate > 0 || x.id == cur_valve.id)
        .count()
        - 1;

    let limits = if total_flows % 2 == 0 {
        let start = total_flows / 2;

        vec![((start, 0), (start, 0))]
    } else {
        let start = total_flows / 2;
        vec![
            ((start - 1, 500), (start + 3, 1000)),
            // ((start, 1000), (start + 2, 1000)),
        ]
    };
    let length = limits.len();
    let paths = limits
        .into_iter()
        .enumerate()
        .map(|(idx, x)| {
            if idx % 10000 == 0 {
                println!("{} of {}", idx, length);
            }
            x
        })
        .flat_map(|(low_limit, high_limit)| {
            println!("{} {:?} {:?}", total_flows, low_limit, high_limit);

            let low = max_pressure_rec(cur_valve, graph, 26, &vec![], low_limit.0)
                .into_iter()
                .filter(|x| x.0 > low_limit.1)
                .collect_vec();
            let high = max_pressure_rec(cur_valve, graph, 26, &vec![], high_limit.0)
                .into_iter()
                .filter(|x| x.0 > high_limit.1)
                .collect_vec();

            vec![low, high]
        })
        .multi_cartesian_product();
    // .filter(|x| {
    //     let vec_len = x.iter().fold(0, |acc, cur| acc + cur.1.len());
    //     vec_len == total_flows + 2
    // });

    let length = paths.clone().count();
    println!("length - {}", length);

    let stuff = paths
        .enumerate()
        .map(|(idx, x)| {
            if idx % 1000000 == 0 {
                println!("{} of {} - {}%", idx, length, (idx * 100) / length);
            }
            x
        })
        .map(|vec| {
            vec.iter()
                .fold((0, vec![]), |(pressure, mut v), (cur_p, cur_v)| {
                    v.append(&mut cur_v.clone());
                    (pressure + cur_p, v)
                })
        })
        .filter(|x| x.0 > 1700)
        .enumerate()
        .map(|(idx, x)| {
            if idx % 1000000 == 0 {
                println!("Passed filter");
            }
            x
        })
        .filter(|(_, x)| {
            x.iter()
                .filter(|x| *x.clone() != cur_valve.id.clone())
                .all_unique()
        })
        .collect_vec();
    // let (pressure, vec) = results.iter().max_by_key(|x| x.0).unwrap();
    // println!("{:#?}", stuff);
    println!("{:#?}", stuff.len());
    let x = stuff.iter().max_by_key(|x| x.0).unwrap();
    println!("{:?}", x.1);
    x.0
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
    1,
    1,
    solution_2,
    1,
    1,
    "src",
    "year_2022",
    "day_17"
);
