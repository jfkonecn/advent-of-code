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

    // for (x, y) in packets {
    //     println!("{:?}", x);
    //     println!("{:?}", y);
    //     println!("");
    // }
    1
}

pub fn solution_2(file_contents: String) -> usize {
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
    "day_13"
);
