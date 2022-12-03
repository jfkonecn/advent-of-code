use crate::common::inputs::challenge_test_suite;

pub fn solution_1(file_contents: String) -> i64 {
    file_contents
        .split("\n\n")
        .map(|str| {
            str.split('\n')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .max()
        .unwrap()
}

pub fn solution_2(file_contents: String) -> i64 {
    let mut vec = file_contents
        .split("\n\n")
        .map(|str| {
            str.split('\n')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .collect::<Vec<i64>>();
    vec.sort_by(|a, b| b.cmp(a));
    vec.truncate(3);
    vec.iter().sum()
}

challenge_test_suite!(
    solution_1,
    24000,
    69206,
    solution_2,
    45000,
    197400,
    "src",
    "year_2022",
    "day_1"
);
