extern crate clap;

mod advent01;
mod advent02;
mod advent03;
mod advent04;
mod advent05;
mod advent06;
mod advent07;
use advent01::start01;
use advent02::start02;
use advent03::start03;
use advent04::start04;
use advent05::start05;
use advent06::start06;
use advent07::start07;
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
            .possible_value("4b")
            .possible_value("5a")
            .possible_value("6a")
            .possible_value("6b")
            .possible_value("7a")
            .possible_value("7b")
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
        Some("4b") => start04::start_b(),
        Some("5a") => start05::start_a(),
        Some("6a") => start06::start_a(),
        Some("6b") => start06::start_b(),
        Some("7a") => start07::start_a(),
        Some("7b") => start07::start_b(),
        Some(invalid_input) => eprintln!("'{}' is not a valid solution", invalid_input),
        None => eprintln!("missing arguments")
    }


}
