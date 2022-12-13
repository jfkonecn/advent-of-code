use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::cmp::*;
use std::collections::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum NodeType {
    Start,
    Middle,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    start: Node,
    end: Node,
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
                    (NodeType::Start, 1)
                } else if 'E' as usize == c_num {
                    raw_end = (row, col);
                    (NodeType::End, 26)
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
            let distance_to_end = (distance_to_end * 1000f64) as usize;
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
            start: nodes.get(&raw_start).unwrap().clone(),
            end: nodes.get(&raw_end).unwrap().clone(),
            nodes,
        }
    }
}
fn shortest_path(graph: &Graph, start: &Node) -> Option<(Vec<Node>, usize)> {
    pathfinding::directed::dijkstra::dijkstra(
        // pathfinding::directed::astar::astar(
        start,
        |node: &Node| {
            let node = node.clone();
            let mut edges = vec![(node.row + 1, node.col), (node.row, node.col + 1)];
            if node.row > 0 {
                edges.push((node.row - 1, node.col));
            }
            if node.col > 0 {
                edges.push((node.row, node.col - 1));
            }
            let edges = edges
                .iter()
                .filter_map(move |key| {
                    let cur_node_id_opt = graph.nodes.get(&key);
                    if let Some(_) = cur_node_id_opt {
                        let cur_node = graph.nodes.get(&key).unwrap();
                        if cur_node.height.abs_diff(node.height) <= 1
                            || cur_node.height < node.height
                        {
                            Some((cur_node.clone(), 100000))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect_vec();
            edges
        },
        |x| x.node_type == NodeType::End,
    )
}

pub fn solution_1(file_contents: String) -> usize {
    let graph = Graph::from(file_contents);
    let (vec, _) = shortest_path(&graph, &graph.start).unwrap();
    vec.len() - 1
}

pub fn solution_2(file_contents: String) -> usize {
    let graph = Graph::from(file_contents);
    let shortest = graph
        .nodes
        .iter()
        .filter(|(_, x)| x.height == 1)
        .map(|(_, x)| x.clone());
    let mut dist = usize::MAX;
    for short in shortest {
        let opt = shortest_path(&graph, &short);

        if let Some((vec, _)) = opt {
            dist = dist.min(vec.len() - 1);
        }
    }
    dist
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
    "day_13"
);
