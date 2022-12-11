use crate::common::inputs::challenge_test_suite;
use itertools::Itertools;
use num_bigint::BigUint;
use std::collections::*;

struct Monkey {
    items: VecDeque<BigUint>,
    inspect: Box<dyn Fn(BigUint) -> BigUint>,
    throw_to: Box<dyn Fn(&BigUint) -> usize>,
    inspections: usize,
}

enum Operation {
    Add,
    Multiply,
}

impl From<&str> for Monkey {
    fn from(str: &str) -> Self {
        let mut str_lines = str.split('\n');

        str_lines.next();
        let items = VecDeque::from_iter(
            str_lines
                .next()
                .unwrap()
                .split(':')
                .last()
                .unwrap()
                .split(',')
                .map(|x| x.trim().parse().unwrap()),
        );

        let inspect: Box<dyn Fn(BigUint) -> BigUint> = {
            let (opt, num, raw_str) = {
                let str = str_lines.next().unwrap().split('=').last().unwrap();
                let (opt, num_str) = if str.contains('*') {
                    (Operation::Multiply, str.split('*').last().unwrap())
                } else if str.contains('+') {
                    (Operation::Add, str.split('+').last().unwrap())
                } else {
                    unreachable!("unknown operator {}", str);
                };
                let num_str = num_str.trim();
                let num_result: Result<usize, _> = num_str.parse();
                (opt, num_result, num_str)
            };
            match (opt, num, raw_str) {
                (Operation::Add, Ok(num), _) => Box::new(move |x| x + num),
                (Operation::Add, Err(_), "old") => Box::new(|x| &x + &x),
                (Operation::Multiply, Ok(num), _) => Box::new(move |x| x * num),
                (Operation::Multiply, Err(_), "old") => Box::new(|x| x.pow(2)),
                _ => unimplemented!("unknown operation \"{}\"", raw_str),
            }
        };

        let throw_to: Box<dyn Fn(&BigUint) -> usize> = {
            let divide_by: usize = str_lines
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse()
                .unwrap();
            let true_monkey: usize = str_lines
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse()
                .unwrap();
            let false_monkey: usize = str_lines
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse()
                .unwrap();

            Box::new(move |x| {
                if x % divide_by == BigUint::from(0usize) {
                    true_monkey
                } else {
                    false_monkey
                }
            })
        };

        Monkey {
            items,
            inspect,
            throw_to,
            inspections: 0,
        }
    }
}

fn parse_monkeys<'a>(file_contents: String) -> Vec<Monkey> {
    file_contents
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map_into()
        .collect_vec()
}

fn run_solution(file_contents: String, rounds: usize, divide_by: usize) -> usize {
    let mut monkeys = parse_monkeys(file_contents);
    let mut monkey_items = monkeys.iter().map(|x| x.items.clone()).collect_vec();
    for i in 0..rounds {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            let items = monkey_items.get_mut(idx).unwrap();
            let mut actions = VecDeque::new();
            println!("monkey {}", idx);
            while let Some(item) = items.pop_front() {
                let item = (monkey.inspect)(item) / divide_by;
                let throw_to = (monkey.throw_to)(&item);
                monkey.inspections += 1;
                actions.push_back((item, throw_to));
            }
            for (item, throw_to) in actions {
                monkey_items.get_mut(throw_to).unwrap().push_back(item);
            }
        }
        println!("Round {} of {}", i + 1, rounds);
    }

    let vec = monkeys.iter().map(|x| x.inspections).collect_vec();
    println!("{:?}", vec);

    monkeys
        .iter()
        .map(|x| x.inspections)
        .sorted()
        .rev()
        .take(2)
        .product()
}

pub fn solution_1(file_contents: String) -> usize {
    run_solution(file_contents, 20, 3)
}

pub fn solution_2(file_contents: String) -> usize {
    run_solution(file_contents, 10000, 1)
}

challenge_test_suite!(
    solution_1,
    10605,
    182293,
    solution_2,
    2713310158,
    1,
    "src",
    "year_2022",
    "day_11"
);
