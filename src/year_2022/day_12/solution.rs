use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::collections::*;

#[derive(Debug, Clone)]
enum NodeType {
    Start,
    Middle,
    End,
}

#[derive(Debug, Clone)]
struct Node {
    row: usize,
    col: usize,
    height: usize,
    distance_to_end: usize,
    node_type: NodeType,
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<(usize, usize), Node>,
    start_position: (usize, usize),
}

enum Operation {
    Add,
    Multiply,
}

impl From<String> for Graph {
    fn from(str: String) -> Self {
        let str_lines = str.split('\n').filter(|x| !x.is_empty());
        let mut raw_start = (0, 0);
        let mut raw_nodes = HashMap::new();
        for (row, str) in str_lines.enumerate() {
            for (col, c_num) in str.chars().map(|x| x as usize).enumerate() {
                let node_type = if 'S' as usize == c_num {
                    raw_start = (row, col);
                    NodeType::Start
                } else if 'E' as usize == c_num {
                    NodeType::End
                } else {
                    NodeType::Middle
                };
                raw_nodes.insert((row, col), (c_num, node_type));
            }
        }

        let nodes = HashMap::new();

        for raw_node in &raw_nodes {
            let t = raw_nodes.get(&(0, 0)).unwrap();
        }
        Graph {
            nodes,
            start_position: raw_start,
        }
    }
}

pub fn solution_1(file_contents: String) -> usize {
    let graph = Graph::from(file_contents);
    println!("{:?}", graph);
    1
}

pub fn solution_2(file_contents: String) -> usize {
    let graph = Graph::from(file_contents);
    println!("{:?}", graph);
    1
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
    "day_12"
);
