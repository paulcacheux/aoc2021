use clap::Parser;

mod aoc2021;
mod helpers;

use helpers::Results;

#[derive(Parser)]
#[clap(version = "1.0", author = "Paul C. <paulcacheux@gmail.com>")]
struct Options {
    /// Use test input
    #[clap(long)]
    test: bool,
    /// Advent year
    #[clap(long, default_value = "2021")]
    year: u32,
    /// Advent day
    #[clap(long)]
    day: u32,
}

fn main() {
    let opts = Options::parse();

    let (input_path, results) = if opts.test {
        (
            format!("./inputs/{}/day{}_test.txt", opts.year, opts.day),
            None,
        )
    } else {
        (
            format!("./inputs/{}/day{}.txt", opts.year, opts.day),
            Some(Results::parse(opts.year).expect("Failed to parse results")),
        )
    };
    let input = std::fs::read_to_string(input_path).expect("failed to read input");

    aoc2021::run_solution_for_day(opts.day, &input, results);
}
