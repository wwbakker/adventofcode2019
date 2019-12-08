extern crate clap;

mod advent01;
use advent01::start01;
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
            .required(true)
        );
   let matches: ArgMatches = command_line_information.get_matches();

    let solution_input:Option<&str> = matches.value_of("solution");
    match solution_input {
        Some("1a") => start01::start_a(),
        Some("1b") => start01::start_b(),
        Some(invalid_input) => eprintln!("'{}' is not a valid solution", invalid_input),
        None => eprintln!("missing arguments")
    }


}
