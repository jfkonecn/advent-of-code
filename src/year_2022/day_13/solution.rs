use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::cmp::*;
use std::collections::*;
use std::fmt::format;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PacketList {
    list: Vec<PacketIndex>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PacketIndex {
    List(PacketList),
    Num(usize),
}

fn cmp_rec(first: &PacketIndex, second: &PacketIndex) -> Ordering {
    match (first, second) {
        (PacketIndex::Num(x), PacketIndex::Num(y)) => {
            if x < y {
                Ordering::Less
            } else if x > y {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        (PacketIndex::List(x_list), PacketIndex::List(y_list)) => {
            let mut x_iter = x_list.list.iter();
            let mut y_iter = y_list.list.iter();
            let len_order_result = if x_iter.len() == y_iter.len() {
                Ordering::Equal
            } else if x_iter.len() > y_iter.len() {
                Ordering::Greater
            } else {
                Ordering::Less
            };
            while let (Some(x), Some(y)) = (x_iter.next(), y_iter.next()) {
                let order_result = cmp_rec(x, y);
                if order_result != Ordering::Equal {
                    return order_result;
                }
            }
            len_order_result
        }
        (PacketIndex::List(_), PacketIndex::Num(_)) => cmp_rec(
            first,
            &PacketIndex::List(PacketList {
                list: vec![second.clone()],
            }),
        ),
        (PacketIndex::Num(_), PacketIndex::List(_)) => cmp_rec(
            &PacketIndex::List(PacketList {
                list: vec![first.clone()],
            }),
            second,
        ),
    }
}

impl PartialOrd for PacketList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketList {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_rec(
            &PacketIndex::List(self.clone()),
            &PacketIndex::List(other.clone()),
        )
    }
}

impl From<&str> for PacketList {
    fn from(str: &str) -> Self {
        let mut list = PacketList { list: vec![] };
        let mut stack: Vec<PacketList> = vec![];
        let mut char_iter = str.chars();
        while let Some(c) = char_iter.next() {
            let c = if c.is_numeric() {
                let mut str = c.to_string().to_owned();
                loop {
                    let c = char_iter.next().unwrap();
                    if c.is_numeric() {
                        str.push(c);
                    } else {
                        let num: usize = str.parse().unwrap();
                        stack.last_mut().unwrap().list.push(PacketIndex::Num(num));
                        break c;
                    }
                }
            } else {
                c
            };
            if c == '[' {
                stack.push(PacketList { list: vec![] });
            } else if c == ']' {
                let cur_list = stack.pop().unwrap();
                if let Some(pre_list) = stack.last_mut() {
                    pre_list.list.push(PacketIndex::List(cur_list));
                } else {
                    list = cur_list;
                }
            }
        }
        list
    }
}

fn parse_packets(str: String) -> Vec<(PacketList, PacketList)> {
    str.split("\n\n")
        .map(|x| {
            let mut iter = x.split('\n');
            let first = iter.next().unwrap().into();
            let second = iter.next().unwrap().into();
            (first, second)
        })
        .collect_vec()
}

pub fn solution_1(file_contents: String) -> usize {
    let packets = parse_packets(file_contents);
    packets
        .iter()
        .enumerate()
        .filter(|(_, (x, y))| x.cmp(y) != Ordering::Greater)
        .map(|(x, _)| x + 1)
        .sum()
}

pub fn solution_2(file_contents: String) -> usize {
    let mut packets = file_contents
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| -> PacketList { x.into() })
        .collect_vec();
    let starter = PacketList {
        list: vec![PacketIndex::List(PacketList {
            list: vec![PacketIndex::Num(2)],
        })],
    };
    let ender = PacketList {
        list: vec![PacketIndex::List(PacketList {
            list: vec![PacketIndex::Num(6)],
        })],
    };
    packets.push(starter.clone());
    packets.push(ender.clone());
    packets.sort();
    let start_idx = packets
        .iter()
        .enumerate()
        .find(|(_, x)| x.clone() == &starter)
        .unwrap()
        .0
        + 1;
    let end_idx = packets
        .iter()
        .enumerate()
        .find(|(_, x)| x.clone() == &ender)
        .unwrap()
        .0
        + 1;
    start_idx * end_idx
}

challenge_test_suite!(
    solution_1,
    13,
    6478,
    solution_2,
    140,
    21922,
    "src",
    "year_2022",
    "day_13"
);
