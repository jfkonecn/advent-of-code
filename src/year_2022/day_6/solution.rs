use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use std::collections::VecDeque;

fn end_of_message_idx(file_contents: String, msg_size: usize) -> usize {
    let start_idx = msg_size - 1;
    let mut queue: VecDeque<char> = file_contents[..start_idx].chars().collect();
    for (idx, c) in file_contents[start_idx..].chars().enumerate() {
        queue.push_back(c);
        let count = queue.iter().unique().count();
        if count == msg_size {
            return idx + msg_size;
        }
        queue.pop_front();
    }
    unreachable!("Something went wrong")
}

pub fn solution_1(file_contents: String) -> usize {
    end_of_message_idx(file_contents, 4)
}

pub fn solution_2(file_contents: String) -> usize {
    end_of_message_idx(file_contents, 14)
}

challenge_test_suite!(
    solution_1,
    10,
    1802,
    solution_2,
    29,
    3551,
    "src",
    "year_2022",
    "day_6"
);
