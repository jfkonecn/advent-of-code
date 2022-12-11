use common::inputs::get_challenge;

use crate::year_2022::day_11::solution::run_solution;

mod common;
mod year_2022;

fn main() {
    let contents = get_challenge!("src", "year_2022", "day_11", "real_inputs", "input.txt",);
    let answer = run_solution(contents, 10000, 1);
    println!("answer {}", answer);
}
