use std::io;
use std::collections::LinkedList;

pub fn empty_test_input_fn() -> i32 { panic!("input is read, which was not expected") }

pub fn empty_test_output_fn(_: i32) -> () {}

pub fn read_int_from_command_line() -> i32 {
    println!("Please input a number.");
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

pub fn print_output_to_command_line(output : i32) -> () {
    println!("output: {}", output);
}

pub fn input_of(i: i32) -> impl Fn() -> i32 { move || i }

pub struct InputList {
    input : LinkedList<i32>
}

impl InputList {
    pub fn create_input_fn(&mut self) -> impl FnMut() -> i32 + '_ {
        move || self.input.pop_front().unwrap()
    }

    pub fn create_empty() -> InputList {
        InputList {
            input: LinkedList::new()
        }
    }

    pub fn push(&mut self, v : i32) {
        self.input.push_back(v);
    }
}

pub struct OutputList {
    output : LinkedList<i32>
}
impl OutputList {
    pub fn create_output_fn(&mut self) -> impl FnMut(i32) -> () + '_ {
        move |i| self.output.push_back(i)
    }
    pub fn create_empty() -> OutputList {
        OutputList {
            output: LinkedList::new()
        }
    }
    pub fn pop(&mut self) -> i32 {
        self.output.pop_front().unwrap()
    }
}