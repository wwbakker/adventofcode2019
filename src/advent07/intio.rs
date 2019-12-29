//use std::io;
use std::collections::LinkedList;
use crate::advent07::model::PossiblyInput;

pub fn empty_test_fn() -> SingleValue { SingleValue { value: 0 } }

pub fn input_of(i: i32) -> SingleValue { SingleValue { value: i } }


pub trait InputMethod {
    fn read(&mut self) -> PossiblyInput;
}

pub trait OutputMethod {
    fn write(&mut self, _: i32);
}

#[derive(Debug)]
pub struct IoList {
    list: LinkedList<i32>
}

impl IoList {
    pub fn create_empty() -> IoList {
        IoList {
            list: LinkedList::new()
        }
    }

    pub fn push(&mut self, v: i32) {
        self.list.push_back(v);
    }

    pub fn pop(&mut self) -> i32 {
        self.list.pop_front().unwrap()
    }

//    pub fn peek(&self) -> Option<&i32> {
//        self.list.back()
//    }

}

impl InputMethod for IoList {
    fn read(&mut self) -> PossiblyInput {
        match self.list.pop_front() {
            Some(v) => PossiblyInput::Available(v),
            None => PossiblyInput::Unavailable
        }
    }
}

impl OutputMethod for IoList {
    fn write(&mut self, value: i32) {
        self.list.push_back(value)
    }
}

#[derive(Debug)]
pub struct SingleValue {
    pub value: i32
}

impl InputMethod for SingleValue {
    fn read(&mut self) -> PossiblyInput {
        PossiblyInput::Available(self.value)
    }
}

impl OutputMethod for SingleValue {
    fn write(&mut self, value: i32) {
        self.value = value
    }
}