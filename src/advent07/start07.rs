use super::code_reading;
use super::intcode;
use std::borrow::BorrowMut;
use crate::advent07::intio::{read_int_from_command_line, print_output_to_command_line};

pub fn start_a() {
    match code_reading::read("input/advent05/input.txt") {
        Ok(mut v) => {
            let code = v.as_mut();
            intcode::exec(code, &read_int_from_command_line, print_output_to_command_line.borrow_mut());
            println!("Successfully executed intcode");
            println!("{:?}", code)

        },
        Err(e) => eprintln!("Error occurred: {}", e.to_string().as_str())
    }
}

