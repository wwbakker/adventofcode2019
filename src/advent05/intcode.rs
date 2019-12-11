use crate::advent05::model::Operation;
use crate::advent05::model::NextProgramAction;

pub fn exec(code: &mut Vec<i32>,
            read_int_function: &dyn Fn() -> i32,
            output_function: &dyn Fn(i32) -> ()) {
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
                                      output_function: &dyn Fn(i32) -> ()) -> (NextProgramAction, i32) {
    let operation = Operation::parse_at_program_counter(code, program_counter);
    operation.execute(code, program_counter, read_int_function, output_function)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_test_input_fn() -> i32 { 0 }
    fn empty_test_output_fn(_ : i32) -> () { }

    #[test]
    fn test_single_operation() {
        let mut immutable_code = vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50);
        let code = immutable_code.as_mut();
        let expected_result : Vec<i32> = vec!(1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50);
//        let (first_command_result, _) =
            parse_and_execute_single_operation(code, 0, &empty_test_input_fn, &empty_test_output_fn);
        assert_eq!(code, &expected_result)
    }

    #[test]
    fn test_complete_execution() {
        let mut immutable_code = vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50);
        let code = immutable_code.as_mut();
        let expected_result: Vec<i32> = vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50);
        exec(code, &empty_test_input_fn, &empty_test_output_fn);
        assert_eq!(code, &expected_result)
    }

    #[test]
    fn test_new_operations() {
        let mut immutable_code = vec!(3,9,8,9,10,9,4,9,99,-1,8);
//        let code = immutable_code.as_mut();
//        let mut test_output = 0;
//        let output_fn: &dyn Fn(i32) -> () = &(|v : i32| { test_output = v; ()});
//        let input_7_fn: &dyn Fn() -> i32 = &(|| 7);
//        let input_8_fn: &dyn Fn() -> i32 = &(|| 8);
//
//        exec(code, input_7_fn, output_fn);
//        assert_eq!(test_output, 0);
//        exec(code, input_8_fn, output_fn);
//        assert_eq!(test_output, 1);
    }
}


