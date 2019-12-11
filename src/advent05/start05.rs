use super::read_ints;
use super::intcode;
use std::io;
use std::borrow::BorrowMut;

pub fn start_a() {
    match read_ints::read("input/advent05/input.txt") {
        Ok(mut v) => {
            let code = v.as_mut();
            intcode::exec(code, &read_int_from_command_line, print_output_to_command_line.borrow_mut());
            println!("Successfully executed intcode");
            println!("{:?}", code)

        },
        Err(e) => eprintln!("Error occurred: {}", e.to_string().as_str())
    }
}

fn read_int_from_command_line() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: Result<i32,_> = input.trim().parse();
    match number {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Could not parse '{}'. Please input an integer, try again ({})", input, e.to_string());
            read_int_from_command_line()
        }
    }
}

fn print_output_to_command_line(output : i32) -> () {
    println!("output: {}", output);
}