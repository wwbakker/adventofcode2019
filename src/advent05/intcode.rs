use crate::advent05::model::Operation;
use crate::advent05::model::NextProgramAction;

pub fn exec(code: &mut Vec<i32>,
            read_int_function: &dyn Fn() -> i32,
            output_function: &mut dyn FnMut(i32) -> ()) {
    let mut program_counter: i32 = 0;
    let mut next_program_action = NextProgramAction::Continue;

    while let NextProgramAction::Continue = next_program_action {
        let (npa, next_instruction) =
            parse_and_execute_single_operation(code, program_counter, read_int_function, output_function);
        program_counter = next_instruction;
        next_program_action = npa;
    }
}

fn parse_and_execute_single_operation(code: &mut Vec<i32>,
                                      program_counter: i32,
                                      read_int_function: &dyn Fn() -> i32,
                                      output_function: &mut dyn FnMut(i32) -> ()) -> (NextProgramAction, i32) {
    let operation = Operation::parse_at_program_counter(code, program_counter);
    operation.execute(code, program_counter, read_int_function, output_function)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;

    fn empty_test_input_fn() -> i32 { 0 }

    fn empty_test_output_fn(_: i32) -> () {}

    fn input_of(i: i32) -> impl Fn() -> i32 { move || i }
//    fn output_of(output: &mut i32) -> impl Fn(i32) -> () { move |o| *output = o }

    #[test]
    fn test_single_operation() {
        let mut code = vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50);
        let expected_result: Vec<i32> = vec!(1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50);
        parse_and_execute_single_operation(&mut code, 0, &empty_test_input_fn, empty_test_output_fn.borrow_mut());
        assert_eq!(&code, &expected_result)
    }

    #[test]
    fn test_complete_execution() {
        let mut code = vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50);
        let expected_result: Vec<i32> = vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50);
        exec(&mut code, &empty_test_input_fn, empty_test_output_fn.borrow_mut());
        assert_eq!(&code, &expected_result)
    }

    #[test]
    fn test_equal_position_mode() {
        let mut code = vec!(3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8);
        let mut test_output = 0;
        {
            let mut write_to_test_output_fn = |v: i32| {
                test_output = v;
                ()
            };
            exec(&mut code, &input_of(7), write_to_test_output_fn.borrow_mut());
        }
        assert_eq!(test_output, 0);
        {
            let mut write_to_test_output_fn = |v: i32| {
                test_output = v;
                ()
            };
            exec(&mut code, &input_of(8), write_to_test_output_fn.borrow_mut());
        }
        assert_eq!(test_output, 1);
    }

    #[test]
    fn test_less_than_position_mode() {
        let mut code = vec!(3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8);

        exec(&mut code, &input_of(7), empty_test_output_fn.borrow_mut());
        assert_eq!(code[9], 1);
        exec(&mut code, &input_of(8), empty_test_output_fn.borrow_mut());
        assert_eq!(code[9], 0);
    }

    #[test]
    fn test_equal_immediate_mode() {
        let mut code = vec!(3, 3, 1108, -1, 8, 3, 4, 3, 99);

        exec(&mut code, &input_of(7), empty_test_output_fn.borrow_mut());
        assert_eq!(code[3], 0);
        exec(&mut code, &input_of(8), empty_test_output_fn.borrow_mut());
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
            exec(&mut code, &input_of(0), write_to_test_output_fn.borrow_mut());
        }
        assert_eq!(test_output, 0);
        {
            let mut write_to_test_output_fn = |v: i32| {
                test_output = v;
                ()
            };
            exec(&mut code, &input_of(-5), write_to_test_output_fn.borrow_mut());
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
            exec(&mut code, &input_of(0), write_to_test_output_fn.borrow_mut());
        }
        assert_eq!(test_output, 0);
        {
            let mut code = vec!(3,3,1105,-1,9,1101,0,0,12,4,12,99,1);
            let mut write_to_test_output_fn = |v: i32| {
                test_output = v;
                ()
            };
            exec(&mut code, &input_of(-5), write_to_test_output_fn.borrow_mut());
        }
        assert_eq!(test_output, 1);
    }
}


