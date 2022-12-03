use crate::common::inputs::challenge_test_suite;
use std::collections::HashSet;

pub fn solution_1(file_contents: String) -> usize {
    file_contents
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|str| {
            let idx = str.len() / 2;
            let a = &str[..idx];
            let b = &str[idx..];
            let mut set = HashSet::new();
            for c in a.chars() {
                set.insert(c);
            }
            for c in b.chars() {
                if set.contains(&c) {
                    return c;
                }
            }
            panic!("no matches between \"{}\" and \"{}\"", a, b)
        })
        .map(|c| {
            let small_a = 'a' as usize;
            let big_a = 'A' as usize;
            let char_num = c as usize;
            let num = if char_num >= small_a {
                char_num - small_a + 1
            } else {
                char_num - big_a + 27
            };
            num
        })
        .sum()
}

pub fn solution_2(file_contents: String) -> usize {
    1
    // file_contents
    //     .split("\n")
    //     .filter(|x| !x.is_empty())
    //     .map(|str| {
    //         let arr = str.split(' ').collect::<Vec<&str>>();
    //         let opponent: Rps = (*arr.get(0).unwrap()).into();
    //         let winner: RpsWinner = (*arr.get(1).unwrap()).into();
    //         let me = should_pick(&opponent, &winner);
    //         usize::from(winner) + usize::from(me)
    //     })
    //     .sum()
}

challenge_test_suite!(
    solution_1,
    157,
    1,
    // solution_2,
    // 12,
    // 11373,
    "src",
    "year_2022",
    "day_3"
);
