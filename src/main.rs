use common::inputs::get_challenge;

use crate::year_2022::day_16::solution::solution_2;

mod common;
mod year_2022;

fn main() {
    let contents = get_challenge!("src", "year_2022", "day_16", "real_inputs", "input.txt",);
    let answer = solution_2(contents);
    println!("answer {}", answer);
}
