use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::cmp::*;
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
}

impl From<String> for Graph {
    fn from(str: String) -> Self {
        let str_lines = str.split('\n').filter(|x| !x.is_empty());
        let mut raw_end = (0, 0);
        let mut raw_nodes = HashMap::new();
        for (row, str) in str_lines.enumerate() {
            for (col, c_num) in str.chars().map(|x| x as usize).enumerate() {
                let (node_type, height) = if 'S' as usize == c_num {
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
        Graph { nodes }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
    distance_to_end: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.distance_to_end))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
    distance_to_goal: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(graph: &Graph) -> Option<usize> {
    let (adj_list, start, goal, start_distance_to_goal) = {
        let vec = graph
            .nodes
            .iter()
            .sorted_by(|(a, _), (b, _)| Ord::cmp(a, b))
            .map(|(_, x)| x)
            .collect_vec();
        let mut adj_list = vec![];
        let mut start = 0;
        let mut start_distance_to_goal = 0;
        let mut goal = 0;

        let mut id_map = HashMap::new();
        for (id, node) in vec.iter().enumerate() {
            id_map.insert((node.row, node.col), id);
        }
        for (node_id, node) in vec.iter().enumerate() {
            if let NodeType::Start = node.node_type {
                start = node_id;
                start_distance_to_goal = node.distance_to_end;
            } else if let NodeType::End = node.node_type {
                goal = node_id;
            }
            let mut edges = vec![(1, 0), (0, 1)];
            if node.row > 0 {
                edges.push((-1, 0));
            }
            if node.col > 0 {
                edges.push((0, -1));
            }

            let edges = edges
                .iter()
                .filter_map(|(x, y)| {
                    let key = (
                        ((node.row as isize) + x) as usize,
                        ((node.col as isize) + y) as usize,
                    );
                    let cur_node_id_opt = id_map.get(&key);
                    if let Some(cur_node_id) = cur_node_id_opt {
                        let cur_node = graph.nodes.get(&key).unwrap();
                        if cur_node.height.abs_diff(node.height) <= 1
                            || cur_node.height < node.height
                        {
                            Some(Edge {
                                node: *cur_node_id,
                                cost: 1,
                                distance_to_goal: cur_node.distance_to_end,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect_vec();

            adj_list.push(edges);
        }
        (adj_list, start, goal, start_distance_to_goal)
    };

    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
        distance_to_end: start_distance_to_goal,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position, .. }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
                distance_to_end: edge.distance_to_goal,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

pub fn solution_1(file_contents: String) -> usize {
    let graph = Graph::from(file_contents);
    shortest_path(&graph).unwrap()
}

pub fn solution_2(file_contents: String) -> usize {
    let graph = Graph::from(file_contents);
    1
}

challenge_test_suite!(
    solution_1,
    31,
    // 352 too high
    0,
    solution_2,
    1,
    1,
    "src",
    "year_2022",
    "day_12"
);
