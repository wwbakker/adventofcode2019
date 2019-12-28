//use std::io;
use std::collections::LinkedList;
use crate::advent07::model::PossiblyInput;

pub fn empty_test_input_fn() -> PossiblyInput { panic!("input is read, which was not expected") }
pub fn empty_test_output_fn(_: i32) -> () {}

//pub fn read_int_from_command_line() -> i32 {
//    println!("Please input a number.");
//    let mut input = String::new();
//    io::stdin().read_line(&mut input).unwrap();
//    let number: Result<i32,_> = input.trim().parse();
//    match number {
//        Ok(i) => i,
//        Err(e) => {
//            eprintln!("Could not parse '{}'. Please input an integer, try again ({})", input, e.to_string());
//            read_int_from_command_line()
//        }
//    }
//}

//pub fn print_output_to_command_line(output : i32) -> () {
//    println!("output: {}", output);
//}

pub fn input_of(i: i32) -> impl Fn() -> PossiblyInput { move || PossiblyInput::Available(i) }

#[derive(Debug)]
pub struct IoList {
    list: LinkedList<i32>
}

impl<'a> IoList {

    pub fn create_empty() -> IoList {
        IoList {
            list: LinkedList::new()
        }
    }

    pub fn push(&mut self, v : i32) {
        self.list.push_back(v);
    }

    pub fn pop(&mut self) -> i32 {
        self.list.pop_front().unwrap()
    }

    pub fn peek(&self) -> Option<&i32> {
        self.list.back()
    }

    pub fn create_input_fn(&'a mut self) -> impl FnMut() -> PossiblyInput + 'a {
        || match self.list.pop_front() {
            Some(v) => PossiblyInput::Available(v),
            None => PossiblyInput::Unavailable
        }
    }

    pub fn create_output_fn(&'a mut self) -> impl FnMut(i32) -> () + 'a {
        |i| self.list.push_back(i)
    }
}