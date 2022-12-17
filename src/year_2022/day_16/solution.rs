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
    let mut visited = HashMap::new();
    let mut stack = vec![(raw_valve, 0)];

    while let Some((cur_valve, cur_distance)) = stack.pop() {
        visited.insert(cur_valve.id.clone(), (cur_distance, cur_valve.flow_rate));
        let cur_distance = cur_distance + 1;
        for tunnel_id in cur_valve.tunnels.iter() {
            if let Some(mut x) = visited.get_mut(tunnel_id) {
                x.0 = x.0.min(cur_distance);
            } else {
                let new_valve = graph.iter().find(|x| &x.id == tunnel_id).unwrap();
                stack.push((new_valve, cur_distance));
            }
        }
    }
    visited
        .into_iter()
        .map(|(key, (distance, flow_rate))| Tunnel {
            distance,
            flow_rate,
            to: key,
        })
        .filter(|x| x.distance > 0 && x.flow_rate > 0)
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

    let t = cur_valve
        .tunnels
        .iter()
        .filter(|x| time_left > x.distance + 1 && !visited.contains(&x.to))
        .flat_map(|tunnel| {
            let next_valve = graph.iter().find(|x| x.id == tunnel.to).unwrap();
            let time_left = time_left - tunnel.distance - 1;

            if visited.len() == 2 && cur_valve.id == "DD" {
                println!("exploring {}", tunnel.to);
            }
            let t = max_pressure_rec(next_valve, graph, time_left, &visited)
                .into_iter()
                .map(|(pressure, vec)| {
                    let pressure_delta = time_left * tunnel.flow_rate;
                    // if visited.len() == 4 && tunnel.to == "HH" && cur_valve.id == "JJ" {
                    // if visited.len() == 1 && tunnel.to == "DD" && cur_valve.id == "AA" {
                    //     if visited.len() == 1 && tunnel.to == "DD" {
                    //     // if visited.len() == 2 && tunnel.to == "BB" && cur_valve.id == "DD" {
                    //         println!("From {} go to {} arrive at {} pressure released by valve {} ({}, {:?})",
                    //     cur_valve.id, tunnel.to, time_left, pressure_delta, tunnel.distance, visited,
                    // );
                    //     }
                    let pressure = pressure + pressure_delta;
                    (pressure, vec.clone())
                })
                .collect_vec();
            if visited.len() == 2 && cur_valve.id == "DD" && tunnel.to == "BB" {
                println!("dd result {:?}", t);
            }
            t
        })
        .collect_vec();
    if cur_valve.id == "DD" && visited.len() == 2 {
        println!("{:?}", t);
    }
    if t.len() == 0 {
        return vec![(0, visited)];
    } else {
        t
    }
}

fn max_pressure(cur_valve: &Valve, graph: &Vec<Valve>) -> usize {
    let results = max_pressure_rec(cur_valve, graph, 30, &vec![]);
    // println!("{:#?}", results);
    let (pressure, vec) = results.iter().max_by_key(|x| x.0).unwrap();
    println!("{:#?}", vec);
    *pressure

    // let mut visited = HashMap::new();
    // let mut stack = vec![(cur_valve, 30, 0)];

    // while let Some((cur_valve, time_left, pressure_released)) = stack.pop() {
    //     let (time_left, pressure_released) = if time_left > 1 {
    //         let time_left = time_left - 1;
    //         let pressure_released = pressure_released + (cur_valve.flow_rate * time_left);
    //         (time_left, pressure_released)
    //     } else {
    //         (time_left, pressure_released)
    //     };
    //     visited.insert(cur_valve.id.clone(), pressure_released);
    //     for tunnel in cur_valve.tunnels.iter() {
    //         if let Some(mut x) = visited.get_mut(&tunnel.to) {
    //             x = &mut pressure_released.max(*x);
    //         } else {
    //             let new_valve = graph.iter().find(|x| x.id == tunnel.to).unwrap();
    //             stack.push((new_valve, time_left, pressure_released));
    //         }
    //     }
    // }
    // *visited.iter().map(|(_, x)| x).max().unwrap()
}

pub fn solution_1(file_contents: String) -> usize {
    let raw_valves = parse_raw_valves(file_contents);
    let valves = to_valves(&raw_valves);
    println!("{:#?}", valves);
    valves
        .iter()
        .filter(|x| x.id == "AA")
        .map(|x| {
            println!("##################################################");
            println!("##################################################");
            let temp = max_pressure(x, &valves);
            println!("##################################################");
            println!("##################################################");
            temp
        })
        .max()
        .unwrap()
}

pub fn solution_2(file_contents: String) -> usize {
    0
}

challenge_test_suite!(
    solution_1,
    1,
    // 1469 is too low
    1,
    solution_2,
    1,
    1,
    "src",
    "year_2022",
    "day_16"
);
