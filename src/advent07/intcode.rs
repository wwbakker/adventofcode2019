use crate::advent07::model::{Operation, NextProgramAction, IoEvent};

pub struct IntCode {
    code: Vec<i32>,
    program_counter : i32,
    next_program_action : NextProgramAction,
    latest_io_event : Option<IoEvent>
}

impl IntCode {

    pub fn create(code_ref : &Vec<i32>) -> IntCode {
        IntCode {
            code: code_ref.clone(),
            next_program_action: NextProgramAction::Continue { next_instruction: 0 },
            latest_io_event: Option::None
        }
    }

    pub fn exec(&mut self,
                read_int_function: &mut dyn FnMut() -> i32,
                output_function: &mut dyn FnMut(i32) -> ()) {

        loop {
            self.parse_and_execute_single_operation();

            match self.next_program_action {
                NextProgramAction::Continue { next_instruction} => program_counter = next_instruction,
                NextProgramAction::Halt => break
            }
            match self.latest_io_event {
                Some(io_event) =>
                    match io_event {
                        IoEvent::AwaitingInput { input_destination_address} => code[input_destination_address as usize] = read_int_function(),
                        IoEvent::Output { value } => output_function(value),
                    },
                None => ()
            }
        }
    }

    fn parse_and_execute_single_operation(&mut self) {
        match self.next_program_action {
            NextProgramAction::Continue {next_instruction} => {
                let operation = Operation::parse_at_program_counter(&self.code, self.program_counter);
                let (npa, ioevent) = operation.execute(code, program_counter);
                self.next_program_action = npa;
                self.latest_io_event = ioevent;
            },
            _ => panic!("Cannot continue, intcode halted")
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;
    use crate::advent07::intio::{empty_test_input_fn, empty_test_output_fn, input_of, IoList};

    #[test]
    fn test_single_operation() {
        let mut code = vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50);
        let expected_result: Vec<i32> = vec!(1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50);
        parse_and_execute_single_operation(&mut code, 0);
        assert_eq!(&code, &expected_result)
    }

    #[test]
    fn test_complete_execution() {
        let mut code = vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50);
        let expected_result: Vec<i32> = vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50);
        exec(&mut code, &mut empty_test_input_fn, empty_test_output_fn.borrow_mut());
        assert_eq!(&code, &expected_result)
    }

    #[test]
    fn test_equal_position_mode() {
        let mut code = vec!(3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8);
        {
            let mut output_list = IoList::create_empty();
            exec(&mut code, &mut input_of(7), &mut output_list.create_output_fn());
            assert_eq!(output_list.pop(), 0);
        }
        {
            let mut output_list = IoList::create_empty();
            exec(&mut code, &mut input_of(8), &mut output_list.create_output_fn());
            assert_eq!(output_list.pop(), 1);
        }
    }

    #[test]
    fn test_less_than_position_mode() {
        let mut code = vec!(3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8);

        exec(&mut code, &mut input_of(7), empty_test_output_fn.borrow_mut());
        assert_eq!(code[9], 1);
        exec(&mut code, &mut input_of(8), empty_test_output_fn.borrow_mut());
        assert_eq!(code[9], 0);
    }

    #[test]
    fn test_equal_immediate_mode() {
        let mut code = vec!(3, 3, 1108, -1, 8, 3, 4, 3, 99);

        exec(&mut code, &mut input_of(7), empty_test_output_fn.borrow_mut());
        assert_eq!(code[3], 0);
        exec(&mut code, &mut input_of(8), empty_test_output_fn.borrow_mut());
        assert_eq!(code[3], 1);
    }

    #[test]
    fn test_jump_case_position_mode() {
        let mut code = vec!(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9);
        let mut test_output = 0;
        {
            let mut write_to_test_output_fn = |v: i32| {
                test_output = v;
                ()
            };
            exec(&mut code, &mut input_of(0), write_to_test_output_fn.borrow_mut());
        }
        assert_eq!(test_output, 0);
        {
            let mut write_to_test_output_fn = |v: i32| {
                test_output = v;
                ()
            };
            exec(&mut code, &mut input_of(-5), write_to_test_output_fn.borrow_mut());
        }
        assert_eq!(test_output, 1);
    }

    #[test]
    fn test_jump_case_immediate_mode() {
        let mut test_output = 0;
        {
            let mut code = vec!(3,3,1105,-1,9,1101,0,0,12,4,12,99,1);
            let mut write_to_test_output_fn = |v: i32| {
                test_output = v;
                ()
            };
            exec(&mut code, &mut input_of(0), write_to_test_output_fn.borrow_mut());
        }
        assert_eq!(test_output, 0);
        {
            let mut code = vec!(3,3,1105,-1,9,1101,0,0,12,4,12,99,1);
            let mut write_to_test_output_fn = |v: i32| {
                test_output = v;
                ()
            };
            exec(&mut code, &mut input_of(-5), write_to_test_output_fn.borrow_mut());
        }
        assert_eq!(test_output, 1);
    }
}


