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
    distance_to_end: f64,
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
        let mut raw_end = (0, 0);
        let mut raw_nodes = HashMap::new();
        for (row, str) in str_lines.enumerate() {
            for (col, c_num) in str.chars().map(|x| x as usize).enumerate() {
                let (node_type, height) = if 'S' as usize == c_num {
                    raw_start = (row, col);
                    (NodeType::Start, 0)
                } else if 'E' as usize == c_num {
                    raw_end = (row, col);
                    (NodeType::End, ('z' as usize - 'a' as usize) + 2)
                } else {
                    (NodeType::Middle, c_num - 'a' as usize + 1)
                };
                raw_nodes.insert((row, col), (height, node_type));
            }
        }

        let mut nodes = HashMap::new();

        for ((row, col), (height, node_type)) in &raw_nodes {
            let (row, col) = (*row, *col);
            let (end_row, end_col) = raw_end;
            let distance_to_end = f64::sqrt(
                f64::powi(row.abs_diff(end_row) as f64, 2)
                    + f64::powi(col.abs_diff(end_col) as f64, 2),
            );
            nodes.insert(
                (row, col),
                Node {
                    col,
                    row,
                    distance_to_end,
                    height: *height,
                    node_type: node_type.clone(),
                },
            );
        }
        Graph {
            nodes,
            start_position: raw_start,
        }
    }
}

pub fn solution_1(file_contents: String) -> usize {
    let graph = Graph::from(file_contents);
    println!("{:#?}", graph);
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
