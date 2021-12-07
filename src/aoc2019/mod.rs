use crate::helpers::{run, Results};
use advent_of_code_traits::days::*;

pub struct Aoc2019;

pub mod day1;

pub fn run_solution_for_day(day: u32, input: &str, results: Option<Results>) {
    let r = results
        .as_ref()
        .and_then(|r| r.results_for_day(day as usize));

    match day {
        1 => run::<Aoc2019, Day1>(input, r),
        _ => unimplemented!("no solution available for that day"),
    }
}