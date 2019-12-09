extern crate clap;

mod advent01;
mod advent02;
mod advent03;
mod advent04;
use advent01::start01;
use advent02::start02;
use advent03::start03;
use advent04::start04;
use clap::{Arg, App, ArgMatches};

fn main() {
    let command_line_information: App = App::new("Wessel's Advent of Code 2019 solutions")
        .author("Wessel W. Bakker <wwbakker@gmail.com>")
        .about("Solutions for https://adventofcode.com/2019/")
        .arg(Arg::with_name("solution")
            .short("s")
            .long("solution")
            .takes_value(true)
            .possible_value("1a")
            .possible_value("1b")
            .possible_value("2a")
            .possible_value("2b")
            .possible_value("3a")
            .possible_value("3b")
            .possible_value("4a")
            .required(true)
        );
   let matches: ArgMatches = command_line_information.get_matches();

    let solution_input:Option<&str> = matches.value_of("solution");
    match solution_input {
        Some("1a") => start01::start_a(),
        Some("1b") => start01::start_b(),
        Some("2a") => start02::start_a(),
        Some("2b") => start02::start_b(),
        Some("3a") => start03::start_a(),
        Some("3b") => start03::start_b(),
        Some("4a") => start04::start_a(),
        Some(invalid_input) => eprintln!("'{}' is not a valid solution", invalid_input),
        None => eprintln!("missing arguments")
    }


}
