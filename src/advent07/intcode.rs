use crate::advent07::model::{Operation, ProgramState, PossiblyInput};

pub struct IntCode {
    pub code: Vec<i32>,
    pub program_state: ProgramState,
    program_counter: i32,
}

impl IntCode {
    pub fn create(code_ref: &Vec<i32>) -> IntCode {
        IntCode {
            code: code_ref.clone(),
            program_state: ProgramState::Ready,
            program_counter: 0,
        }
    }

    pub fn create_non_ref(code: Vec<i32>) -> IntCode {
        IntCode {
            code,
            program_state: ProgramState::Ready,
            program_counter: 0,
        }
    }

    pub fn unhalt(&mut self) {
        match self.program_state {
            ProgramState::AwaitingInput => panic!("Cannot unhalt. Program is awaiting input."),
            _ => {
                self.program_state = ProgramState::Ready;
                self.program_counter = 0;
            }
        }
    }

    pub fn exec(&mut self,
                read_int_function: &mut dyn FnMut() -> PossiblyInput,
                output_function: &mut dyn FnMut(i32) -> ()) {
        if let ProgramState::Halted = self.program_state {
            panic!("Program has halted. Please unhalt before restarting.")
        }

        loop {
            match self.program_state {
                ProgramState::Ready => self.parse_and_execute_single_operation(read_int_function, output_function),
                _ => break
            }
        }
    }

    fn parse_and_execute_single_operation(&mut self,
                                          read_int_function: &mut dyn FnMut() -> PossiblyInput,
                                          output_function: &mut dyn FnMut(i32) -> ()) {
        let operation = Operation::parse_at_program_counter(&self.code, self.program_counter);
        let (new_program_state, next_instruction) =
            operation.execute(&mut self.code, self.program_counter, read_int_function, output_function);
        self.program_state = new_program_state;
        self.program_counter = next_instruction;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;
    use crate::advent07::intio::{empty_test_input_fn, empty_test_output_fn, input_of, IoList};

    #[test]
    fn test_single_operation() {
        let mut intcode = IntCode::create_non_ref(vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50));
        let expected_result: Vec<i32> = vec!(1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50);
        intcode.parse_and_execute_single_operation(&mut empty_test_input_fn, empty_test_output_fn.borrow_mut());
        assert_eq!(&intcode.code, &expected_result)
    }

    #[test]
    fn test_complete_execution() {
        let mut intcode = IntCode::create_non_ref(vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50));
        let expected_result: Vec<i32> = vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50);
        intcode.exec(&mut empty_test_input_fn, empty_test_output_fn.borrow_mut());
        assert_eq!(&intcode.code, &expected_result)
    }

    #[test]
    fn test_equal_position_mode() {
        let code = vec!(3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8);
        {
            let mut intcode = IntCode::create(&code);
            let mut output_list = IoList::create_empty();
            intcode.exec(&mut input_of(7), &mut output_list.create_output_fn());
            assert_eq!(output_list.pop(), 0);
        }
        {
            let mut intcode = IntCode::create(&code);
            let mut output_list = IoList::create_empty();
            intcode.exec(&mut input_of(8), &mut output_list.create_output_fn());
            assert_eq!(output_list.pop(), 1);
        }
    }

    #[test]
    fn test_less_than_position_mode() {
        let mut intcode = IntCode::create_non_ref(vec!(3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8));

        intcode.exec(&mut input_of(7), empty_test_output_fn.borrow_mut());
        assert_eq!(intcode.code[9], 1);
        intcode.unhalt();
        intcode.exec(&mut input_of(8), empty_test_output_fn.borrow_mut());
        assert_eq!(intcode.code[9], 0);
    }

    #[test]
    fn test_equal_immediate_mode() {
        let mut intcode = IntCode::create_non_ref(vec!(3, 3, 1108, -1, 8, 3, 4, 3, 99));
        intcode.exec(&mut input_of(7), empty_test_output_fn.borrow_mut());
        assert_eq!(intcode.code[3], 0);
        intcode.unhalt();
        intcode.exec(&mut input_of(8), empty_test_output_fn.borrow_mut());
        assert_eq!(intcode.code[3], 1);
    }

    #[test]
    fn test_jump_case_position_mode() {
        let mut intcode = IntCode::create_non_ref(vec!(3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9));
        let mut test_output = 0;
        {
            let mut write_to_test_output_fn = |v: i32| {
                test_output = v;
                ()
            };
            intcode.exec(&mut input_of(0), write_to_test_output_fn.borrow_mut());
        }
        intcode.unhalt();
        assert_eq!(test_output, 0);
        {
            let mut write_to_test_output_fn = |v: i32| {
                test_output = v;
                ()
            };
            intcode.exec(&mut input_of(-5), write_to_test_output_fn.borrow_mut());
        }
        assert_eq!(test_output, 1);
    }

    #[test]
    fn test_jump_case_immediate_mode() {
        let mut test_output = 0;
        {
            let mut intcode = IntCode::create_non_ref(vec!(3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1));
            let mut write_to_test_output_fn = |v: i32| {
                test_output = v;
                ()
            };
            intcode.exec(&mut input_of(0), write_to_test_output_fn.borrow_mut());
        }
        assert_eq!(test_output, 0);
        {
            let mut intcode = IntCode::create_non_ref(vec!(3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1));
            let mut write_to_test_output_fn = |v: i32| {
                test_output = v;
                ()
            };
            intcode.exec(&mut input_of(-5), write_to_test_output_fn.borrow_mut());
        }
        assert_eq!(test_output, 1);
    }
}


